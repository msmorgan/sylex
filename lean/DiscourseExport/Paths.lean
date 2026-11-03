import Lean
import SylGen.ReadOrder
import Semantics.Card

/-!
# DiscourseExport.Paths

The expanded core AST of a card, as paths. `walk` reduces a card to its constructor tree (as
`DumpValues.render` does) and rebuilds it as one closed term in which every node of a stepped type
(`steppedTypes`) is its own allocation (`keep`), so the checker's frames name nodes by address
without ambiguity. A path is the sequence of (constructor, field, list indices) from the card root;
an `Option` is transparent.
-/

open Lean Meta

namespace DiscourseExport

universe u

/-- `x`, kept out of closed-term extraction: a node with a field wrapped as `keep slot field` is
allocated where it is evaluated, and `slot` keeps equal nodes from being shared. -/
@[noinline, never_extract] def keep {β : Sort u} (_slot : Nat) (x : β) : β := x

/-- The types the checker steps through a traced frame (`Semantics.Trace`), by frame kind. -/
def steppedTypes : List Name := [
  ``Semantics.Ability, ``Semantics.Action, ``Semantics.NounPhrase, ``Semantics.Predicate,
  ``Semantics.Amount, ``Semantics.Condition, ``Semantics.GameEvent, ``Semantics.StaticSpec,
  ``Semantics.Cost, ``Semantics.Quantity, ``Semantics.Timing, ``Semantics.Location,
  ``Semantics.WouldClause, ``Semantics.Replacement, ``Semantics.MoveRider,
  ``Semantics.TokenSpec, ``Semantics.DeckTrait, ``Semantics.CardFace,
  ``Semantics.SharedLineHalf, ``Semantics.LevelBand]

structure Step where
  ctor : Name
  field : String
  indices : Array Nat := #[]
  deriving BEq, Inhabited

instance : ToJson Step where
  toJson s := Json.arr (#[toJson s.ctor.toString, toJson s.field] ++ s.indices.map toJson)

abbrev Path := Array Step

def Path.isPrefixOf (p q : Path) : Bool :=
  p.size ≤ q.size && (p.zip q).all fun (a, b) => a == b

structure Position where
  path : Path
  /-- The stepped type's short name, as the frame kind names it. -/
  type : String
  ctor : Name
  /-- A pronoun's reach constructor (`bare`, …), or a first or later mention's Binding Identity. -/
  detail : String := ""
  deriving Inhabited

private structure WalkState where
  lets : Array (FVarId × Expr × Expr) := #[]
  /-- Each let's position slot. -/
  slots : Array Nat := #[]
  positions : Array Position := #[]
  ctorFields : Std.HashMap Name (Array (String × Bool)) := {}

private abbrev WalkM := StateT WalkState MetaM

/-- Field names of `ctor` after its parameters, each with whether it is a proof. -/
private def fieldsOf (ctor : Name) : WalkM (Array (String × Bool)) := do
  if let some fs := (← get).ctorFields[ctor]? then return fs
  let info ← getConstInfoCtor ctor
  let fs ← forallTelescope info.type fun binders _ => do
    let mut out := #[]
    for b in binders[info.numParams:] do
      let decl ← b.fvarId!.getDecl
      out := out.push (decl.userName.toString, ← isProp decl.type)
    return out
  modify fun s => { s with ctorFields := s.ctorFields.insert ctor fs }
  return fs

/-- The checker's reading order of `fields` (`SylGen.ReadOrder.fieldOrders`, and a passive
clause's action before its subject). -/
private def readOrder (ctor : Name) (fields : Array (String × Expr)) : MetaM (Array Nat) := do
  let names := ((SylGen.ReadOrder.fieldOrders.find? (·.1 == ctor)).map (·.2)).getD []
  let passive ← match fields.find? (·.1 == "subj") with
    | some (_, subject) =>
      if (fields.find? (·.1 == "action")).isSome then
        pure ((← whnf subject).isAppOf `Semantics.Subject.by)
      else pure false
    | none => pure false
  let names := if passive then ["action", "subj"] ++ names else names
  let idx := fun (n : String) => fields.findIdx? (·.1 == n)
  let first := names.filterMap idx
  let firstArr := first.toArray.toList.eraseDups.toArray
  return firstArr ++ (Array.range fields.size).filter (!firstArr.contains ·)

mutual
  /-- Rebuild the value of field `step` (a list, an option or a node). -/
  private unsafe def walkField (path : Path) (step : Step) (value : Expr) : WalkM Expr := do
    let value ← withTransparency .all <| whnf value
    if value.isAppOfArity ``List.cons 3 then
      let args := value.getAppArgs
      let mut elements := #[]
      let mut rest := value
      while rest.isAppOfArity ``List.cons 3 do
        elements := elements.push rest.getAppArgs[1]!
        rest ← withTransparency .all <| whnf rest.getAppArgs[2]!
      unless rest.isAppOfArity ``List.nil 1 do throwError "residual list tail: {rest}"
      let mut rebuilt := #[]
      for h : i in [:elements.size] do
        rebuilt := rebuilt.push
          (← walkField path { step with indices := step.indices.push i } elements[i])
      let ty := args[0]!
      return rebuilt.foldr (fun x acc => mkApp3 (mkConst ``List.cons value.getAppFn.constLevels!)
        ty x acc) rest
    if value.isAppOfArity ``Option.some 2 then
      return mkApp value.appFn! (← walkField path step value.appArg!)
    walkNode (path.push step) value

  /-- Rebuild `value` (already reduced or not) at `path`. -/
  private unsafe def walkNode (path : Path) (value : Expr) : WalkM Expr := do
    let value ← withTransparency .all <| whnf value
    if value.hasFVar || value.hasMVar then throwError "open value"
    if value.rawNatLit?.isSome then return value
    if let .lit _ := value then return value
    let some name := value.getAppFn.constName? | throwError "residual expression: {value}"
    let .ctorInfo info ← getConstInfo name | throwError "residual constant: {name}"
    let args := value.getAppArgs
    unless args.size == info.numParams + info.numFields do
      throwError "unsaturated constructor: {name}"
    let fs ← fieldsOf name
    let fieldArgs := args.extract info.numParams args.size
    let named := (fs.zip fieldArgs).map fun ((n, _), a) => (n, a)
    let order ← readOrder name named
    let mut rebuilt := fieldArgs
    -- A node's own position precedes its children's, in the checker's reading order.
    let stepped := steppedTypes.contains info.induct
    let slot := (← get).positions.size
    if stepped then
      let detail ← if name == ``Semantics.NounPhrase.pro then
          pure ((← withTransparency .all <| whnf fieldArgs[0]!).getAppFn.constName?.map
            (·.getString!) |>.getD "")
        else if name == ``Semantics.NounPhrase.firstMention ||
            name == ``Semantics.NounPhrase.laterMention then
          match ← withTransparency .all <| whnf fieldArgs[0]! with
          | .lit (.strVal id) => pure id
          | _ => evalExpr String (mkConst ``String) fieldArgs[0]! (safety := .unsafe)
        else pure ""
      let position : Position :=
        { path := path, type := info.induct.getString!, ctor := name, detail := detail }
      modify fun s => { s with positions := s.positions.push position }
    for i in order do
      let (field, proof) := fs[i]!
      unless proof do
        rebuilt := rebuilt.set! i (← walkField path { ctor := name, field } fieldArgs[i]!)
    if stepped then
      if let some i := (Array.range fs.size).find? (fun i => !fs[i]!.2) then
        let ty ← inferType fieldArgs[i]!
        rebuilt := rebuilt.set! i
          (mkApp3 (mkConst ``keep [← getLevel ty]) ty (mkRawNatLit slot) rebuilt[i]!)
    let node := mkAppN value.getAppFn (args.extract 0 info.numParams ++ rebuilt)
    unless stepped do return node
    let type := mkAppN (mkConst info.induct value.getAppFn.constLevels!)
      (args.extract 0 info.numParams)
    let fv ← mkFreshFVarId
    modify fun s => { s with lets := s.lets.push (fv, type, node), slots := s.slots.push slot }
    return .fvar fv
end

/-- A card's positions, and the closed term `(card, addresses)` with `slots`: the evaluated
`addresses[j]` is the node of position `slots[j]`. Positions are numbered parent first, in the
checker's reading order. -/
unsafe def walk (card : Expr) : MetaM (Expr × Array Position × Array Nat) := do
  let (root, state) ← (walkNode #[] card).run {}
  let usize := mkConst ``USize
  let addresses := state.lets.foldr (fun (fv, ty, _) acc =>
    mkApp3 (mkConst ``List.cons [Level.zero]) usize
      (mkApp2 (mkConst ``ptrAddrUnsafe [Level.zero]) ty (.fvar fv)) acc)
    (mkApp (mkConst ``List.nil [Level.zero]) usize)
  let cardType ← inferType card
  let listType := mkApp (mkConst ``List [Level.zero]) usize
  let mut body := mkApp4 (mkConst ``Prod.mk [Level.zero, Level.zero]) cardType listType root
    addresses
  -- `lets` are pushed children first, so each value names only earlier lets.
  for (fv, ty, value) in state.lets.reverse do
    body := .letE `_n ty value (body.abstract #[.fvar fv]) false
  return (body, state.positions, state.slots)

end DiscourseExport
