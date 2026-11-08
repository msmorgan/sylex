import SylGen.Parser
import Semantics.Card
import Semantics.Events
import Semantics.Rules
import Semantics.CallerScope
import Semantics.Registry
import Semantics.Primitives

open Lean Meta
namespace SylGen

def leanWord (s : String) : String :=
  if s ∈ ["this", "in", "from", "by", "or", "exists", "where", "at", "repeat", "seal",
      "class", "as", "end", "for", "if", "of", "then", "while", "with"] then s ++ "_" else s

-- Plugin declarations use `or`; the `or_` spelling belongs to named parameters.
def declarationWord (s : String) : String :=
  if s == "or" then s else leanWord s

def parameterName (p : Parameter) : String :=
  if p.binderName.isEmpty then leanWord p.name else p.binderName

def quoted (s : String) : String := "«" ++ s ++ "»"

def leanName (n : Name) : String :=
  String.intercalate "." (n.components.map fun n => match n with
    | .str _ s => quoted s
    | .num _ i => toString i
    | .anonymous => "")

partial def SortRef.lean (s : SortRef) : String :=
  let name := if s.name ∈ ["List", "Option", "Nat", "Int", "Bool", "String"] then s.name
    else "Semantics." ++ quoted s.name
  if s.arguments.isEmpty then name
  else "(" ++ name ++ " " ++ String.intercalate " " (s.arguments.toList.map SortRef.lean) ++ ")"

partial def sortOf (e : Expr) : MetaM SortRef := do
  let e := if e.getOptParamDefault?.isSome then e.getAppArgs[0]! else e
  let e ← whnf e
  if let some name := e.getAppFn.constName? then
    if name == ``List || name == ``Option || name == `Semantics.Delta then
      return { name := name.getString!, arguments := #[← sortOf e.appArg!] }
    return { name := name.getString! }
  throwError "unsupported field sort {e}"

partial def normalizeSort (pos : Position) (sort : SortRef) : MetaM SortRef := do
  if sort.name ∈ ["Option", "List", "Delta"] then
    unless sort.arguments.size == 1 do throwError pos.error "Option, List and Delta require one sort"
    return { sort with arguments := #[← normalizeSort pos sort.arguments[0]!] }
  unless sort.arguments.isEmpty do throwError pos.error "only Option, List and Delta accept sort arguments"
  let name := if sort.name ∈ ["Nat", "Int", "Bool", "String"] then sort.name.toName
    else `Semantics ++ sort.name.toName
  unless (← getEnv).contains name do throwError pos.error s!"unknown sort '{sort.name}'"
  sortOf (mkConst name)

structure Field where
  name : String
  sort : SortRef
  hasDefault : Bool := false
  emittedName : String := ""
  deriving Inhabited, Repr

structure Signature where
  constant : Name
  fields : Array Field
  result : SortRef
  identity : Option String := none
  /-- Why a call to this declaration is refused: a primitive that builds a constructor reserved
  for generated scope and deeds (`sourceConstructorRefusal`). -/
  reserved : Option String := none
  deriving Inhabited, Repr

def signature (name : Name) (type : Option Expr := none) : MetaM Signature := do
  forallTelescope (type.getD (← getConstInfo name).type) fun args result => do
    let mut fields := #[]
    for arg in args do
      let decl ← arg.fvarId!.getDecl
      if decl.binderInfo.isExplicit then
        fields := fields.push {
          name := decl.userName.toString
          sort := (← sortOf decl.type)
          hasDefault := decl.type.getOptParamDefault?.isSome }
    return {
      constant := name, fields := fields, result := (← sortOf result),
      identity := (Semantics.Registry.referenceIdentities.getState (← getEnv)).find? name }

abbrev Registry := Std.HashMap (String × String) Signature

def constructors : MetaM Registry := do
  let env ← getEnv
  let mut result := {}
  for (name, info) in env.constants do
    if let .ctorInfo _ := info then
      if (`Semantics).isPrefixOf name || name.getPrefix == ``Bool then
        try
          let sig ← signature name
          result := result.insert (name.getString!, sig.result.text) sig
        catch _ => pure ()
  for inner in [`Semantics.Amount, `Semantics.ManaCostTerm] do
    for ctor in [`Semantics.Delta.up, `Semantics.Delta.down, `Semantics.Delta.set] do
      let value := mkApp (mkConst ctor) (mkConst inner)
      let sig ← signature ctor (some (← inferType value))
      result := result.insert (ctor.getString!, sig.result.text) sig
  return result

/-- The Lean constructor a primitive's constructor clause names, at the primitive's result sort.
A clause is its Lean constructor capitalized, except where the Rust AST's variant name differs;
those are listed here by sort and clause. -/
def leanConstructor (sort : SortRef) (clause : String) : String :=
  let renamed := [
    ("Amount", "Literal", "lit"), ("ColorTerm", "Literal", "lit"),
    ("ManaCostTerm", "Literal", "lit"), ("UsageLimit", "UsageLimit", "mk"), ("Mode", "Mode", "mk"),
    ("NounPhrase", "Pronoun", "pro"), ("NounPhrase", "Reflexive", "refl"),
    ("NounPhrase", "Coordination", "coord"), ("Location", "Coordination", "coord"),
    ("Predicate", "Coordination", "coord"), ("Condition", "CompareAmount", "compareAmt"),
    ("CharacteristicEdit", "SwitchPowerToughness", "ptSwitch"), ("PreventCut", "Part", "some"),
    ("ColorSpec", "Colors", "some"), ("Amount", "Nth", "theNth"),
    ("Amount", "Difference", "theDifference"), ("Amount", "Outcome", "theOutcome"),
    ("NounPhrase", "Grantor", "theGrantor"), ("NounPhrase", "Rest", "theRest"),
    ("Quantity", "Last", "theLast"), ("ChoiceRef", "Choice", "theChoice"),
    ("ChoiceRef", "LatestChoice", "theLatestChoice"), ("Comparator", "Equal", "eq"),
    ("Card", "ModalDoubleFaced", "modalDfc"), ("ColorOrColorless", "Color", "of"),
    ("QualityOp", "Add", "adds"), ("QualityOp", "Set", "sets"), ("QualityOp", "Lose", "loses"),
    ("Amount", "Arithmetic", "arith")]
  match renamed.find? (fun (s, c, _) => s == sort.name && c == clause) with
  | some (_, _, lean) => lean
  | none => clause.decapitalize

/-- The constructor a primitive's clause names at `sort`, under its Lean name or, where that is a
Lean keyword, with the trailing `_` Lean needs. -/
def Registry.constructor? (ctors : Registry) (sort : SortRef) (clause : String) : Option Signature :=
  let ctor := leanConstructor sort clause
  ctors[(ctor, sort.text)]? <|> ctors[(leanWord ctor, sort.text)]?

/-- Constructors reserved for generated identities or registry declarations. -/
def sourceConstructorRefusal (name : Name) (legacyScope : Bool := false) : Option String :=
  if name == `Semantics.Ability.keyword then
    some "raw Ability.keyword construction requires an ability declaration"
  else if name ∈ [`Semantics.NounPhrase.firstMention, `Semantics.NounPhrase.laterMention,
      `Semantics.Action.ownScope, `Semantics.Deed.action, `Semantics.NounPhrase.inCaller] then
    if legacyScope then none else
      some (if name.getString! ∈ ["firstMention", "laterMention"] then
        s!"{name.getString!} is reserved for generated parameter identities"
      else s!"{name.getPrefix.getString!}.{name.getString!} is reserved for generated scope and deeds")
  else if name.getString! == "inCaller" && name.getPrefix ∈ [
      `Semantics.Action, `Semantics.Cost, `Semantics.StaticSpec, `Semantics.Replacement,
      `Semantics.Amount, `Semantics.Predicate, `Semantics.Quantity, `Semantics.Location,
      `Semantics.Condition, `Semantics.GameEvent, `Semantics.Ability, `Semantics.TokenSpec,
      `Semantics.MoveRider] then
    some s!"{name.getPrefix.getString!}.inCaller is reserved for generated caller scope"
  else if name.getPrefix ∈ [`Semantics.KeywordLabel, `Semantics.CounterKind,
      `Semantics.DesignationLabel, `Semantics.RelationLabel,
      `Semantics.Subtype, `Semantics.CardType, `Semantics.RegistryDefinition] then
    some s!"raw {name.getPrefix.getString!} construction requires a registry declaration"
  else none

def structureFields (sort : SortRef) : MetaM (Array Field) := do
  let env ← getEnv
  let name := `Semantics ++ sort.name.toName
  let some _ := getStructureInfo? env name
    | throwError "{sort.text} is not a record sort"
  let sig ← signature (← getConstInfoInduct name).ctors.head!
  return sig.fields.map fun f =>
    { f with hasDefault := f.hasDefault ||
        (getDefaultFnForField? env name f.name.toName).isSome }

end SylGen
