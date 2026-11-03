import DumpValues.Sampling
import Semantics.Check

/-!
# DiscourseExport.Fixtures

The closed `<entry> args = rhs` equations over `List Refusal` among the theorems of
`Semantics.Proofs.*`, as (input, expected) pairs: the entry point, its explicit arguments in the
`DumpValues` constructor encoding, and whether the right side accepts, with the set of its refusal
kinds (constructor names). Every other theorem is listed by name in the checklist with why it is
not a fixture; `docs/semantics-spec-catalog.md` covers those.
-/

open Lean Meta

namespace DiscourseExport

/-- The public entry points: the checker's `*.check`, `*.checkAt`, `*.checkAll` and
`*.checkText` functions over `List Refusal` defined under `Semantics.Check`. -/
def isEntryPoint (env : Environment) (fn : Name) : Bool :=
  (`Semantics).isPrefixOf fn &&
    ["check", "checkAt", "checkAll", "checkText"].contains (fn.getString!) &&
    match env.getModuleIdxFor? fn with
    | some idx => (`Semantics.Check).isPrefixOf env.header.moduleNames[idx.toNat]!
    | none => false

structure Fixture where
  theoremName : Name
  module : Name
  entry : Name
  args : Array (String × String)
  accepted : Bool
  refusals : Array String

structure ChecklistEntry where
  theoremName : Name
  module : Name
  reason : String

/-- The catalog's `File.theorem` spelling. -/
def catalogName (module theoremName : Name) : String :=
  s!"{module.getString!}.{(privateToUserName? theoremName |>.getD theoremName).getString!}"

private def refusalKinds (rhs : Expr) : MetaM (Array String) := do
  let mut out := #[]
  let mut rest ← withTransparency .all <| whnf rhs
  while rest.isAppOfArity ``List.cons 3 do
    let head ← withTransparency .all <| whnf rest.getAppArgs[1]!
    let some name := head.getAppFn.constName? | throwError "residual refusal: {head}"
    unless (← getConstInfo name).isCtor do throwError "residual refusal: {name}"
    out := out.push name.getString!
    rest ← withTransparency .all <| whnf rest.getAppArgs[2]!
  unless rest.isAppOfArity ``List.nil 1 do throwError "residual refusal list: {rest}"
  return (out.toList.eraseDups.toArray.qsort (· < ·))

/-- `DumpValues`' type-parameter rendering (private there). -/
private partial def renderType (type : Expr) : MetaM String := do
  let type ← withTransparency .all <| whnf type
  if let .sort level := type then return if level.isZero then "Prop" else "Type"
  let .const name _ := type.getAppFn | throwError "unsupported type parameter: {type}"
  let args ← type.getAppArgs.mapM renderType
  return name.toString ++ if args.isEmpty then ""
    else "(" ++ String.intercalate "," args.toList ++ ")"

/-- `DumpValues.render`, but a computed `String` (`operandKey 0`) is evaluated to its literal
rather than reduced into the string's internal representation. -/
private unsafe def renderValue (value : Expr) : MetaM String := do
  if (← whnf (← inferType value)).isConstOf ``String then
    let text ← evalExpr String (mkConst ``String) value (safety := .unsafe)
    return (Json.str text).compress
  let value ← withTransparency .all <| whnf value
  if value.hasMVar || value.hasFVar then throwError "open value"
  if let some n := value.rawNatLit? then return toString n
  if let .lit (.strVal s) := value then return (Json.str s).compress
  let .const name _ := value.getAppFn | throwError "residual expression: {value}"
  let .ctorInfo info ← getConstInfo name | throwError "residual constant: {name}"
  let args := value.getAppArgs
  unless args.size == info.numParams + info.numFields do
    throwError "unsaturated constructor: {name}"
  let params ← (args.extract 0 info.numParams).mapM renderType
  let params := if params.isEmpty then ""
    else "{" ++ String.intercalate "," params.toList ++ "}"
  let fields ← (args.extract info.numParams args.size).mapM renderValue
  return name.toString ++ params ++ "(" ++ String.intercalate "," fields.toList ++ ")"

/-- Render an argument; only where `DumpValues.render` fails, with computed strings evaluated. -/
private unsafe def renderArgument (value : Expr) : MetaM String := do
  try DumpValues.render value
  catch _ => renderValue value

/-- Classify the theorem `name` of `module`. -/
unsafe def classify (name module : Name) (type : Expr) :
    MetaM (Except ChecklistEntry Fixture) := do
  let other := fun reason => pure (.error { theoremName := name, module := module, reason := reason })
  if type.isForall then return ← other "quantified statement"
  if type.hasLooseBVars || type.hasFVar || type.hasMVar then return ← other "open statement"
  let some (α, lhs, rhs) := type.eq? | return ← other "not an equation"
  unless α == mkApp (mkConst ``List [Level.zero]) (mkConst ``Semantics.Refusal) do
    return ← other "not an equation over List Refusal"
  if lhs.hasLooseBVars || (lhs.find? (·.isLet)).isSome then return ← other "let in statement"
  let some fn := lhs.getAppFn.constName? | return ← other "left side is not an application"
  unless isEntryPoint (← getEnv) fn do return ← other s!"not an entry point: {fn}"
  let info ← getConstInfo fn
  let args ← forallTelescope info.type fun binders _ => do
    let mut out := #[]
    for b in binders, a in lhs.getAppArgs do
      let decl ← b.fvarId!.getDecl
      if decl.binderInfo.isExplicit then
        out := out.push (decl.userName.toString, a)
    return out
  try
    let args ← args.mapM fun (n, a) => return (n, ← renderArgument a)
    let refusals ← refusalKinds rhs
    return .ok {
      theoremName := name, module := module, entry := fn, args := args
      accepted := refusals.isEmpty, refusals := refusals }
  catch e => other s!"unrenderable: {← e.toMessageData.toString}"

/-- Every theorem under `Semantics.Proofs`, as a fixture or a checklist entry. -/
unsafe def collectFixtures : MetaM (Array Fixture × Array ChecklistEntry) := do
  let env ← getEnv
  let mut fixtures := #[]
  let mut checklist := #[]
  let mut theorems : Array (Name × Name × Expr) := #[]
  for (name, info) in env.constants.map₁.toList do
    let .thmInfo thm := info | continue
    let some idx := env.getModuleIdxFor? name | continue
    let module := env.header.moduleNames[idx.toNat]!
    unless (`Semantics.Proofs).isPrefixOf module do continue
    let user := privateToUserName? name |>.getD name
    if user.isInternalDetail then continue
    theorems := theorems.push (name, module, thm.type)
  for (name, module, type) in theorems.qsort (fun a b => a.1.toString < b.1.toString) do
    match ← classify name module type with
    | .ok f => fixtures := fixtures.push f
    | .error c => checklist := checklist.push c
  return (fixtures, checklist)

/-- The `File.theorem` names `docs/semantics-spec-catalog.md` marks FLAG DEFECT: every name in a
marked entry's `from:` clause, and every name in section 5 (Binding identity), whose heading marks
each entry that supplies a pronoun as a twice-used argument. -/
def flaggedNames (catalog : String) : Std.HashSet String := Id.run do
  let mut out : Std.HashSet String := {}
  let mut inSection5 := false
  for line in catalog.splitOn "\n" do
    if line.startsWith "### " then inSection5 := line.startsWith "### 5. "
    unless inSection5 || (line.splitOn "FLAG DEFECT").length > 1 do continue
    let parts := line.splitOn "from: "
    if parts.length < 2 then continue
    let clause := (parts.getLast!.splitOn ". FLAG DEFECT")[0]!
    for name in clause.splitOn "," do
      let name := name.trimAscii.toString
      let name := (name.splitOn " ")[0]!
      let name := if name.endsWith "." then (name.dropEnd 1).toString else name
      if (name.splitOn ".").length == 2 then out := out.insert name
  return out

end DiscourseExport
