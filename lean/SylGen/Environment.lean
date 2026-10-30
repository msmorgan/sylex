import SylGen.Parser
import Semantics.Card
import Semantics.Events
import Semantics.Rules
import Semantics.CallerScope

open Lean Meta
namespace SylGen

def quoted (s : String) : String := "«" ++ s ++ "»"

def leanName (n : Name) : String :=
  String.intercalate "." (n.components.map fun n => quoted n.getString!)

partial def SortRef.lean (s : SortRef) : String :=
  let name := if s.name ∈ ["List", "Option", "Nat", "Int", "Bool", "String"] then s.name
    else "Semantics." ++ quoted s.name
  if s.arguments.isEmpty then name
  else "(" ++ name ++ " " ++ String.intercalate " " (s.arguments.toList.map SortRef.lean) ++ ")"

partial def sortOf (e : Expr) : MetaM SortRef := do
  let e := if e.getOptParamDefault?.isSome then e.getAppArgs[0]! else e
  let e ← whnf e
  if let some name := e.getAppFn.constName? then
    if name == ``List || name == ``Option then
      return { name := name.getString!, arguments := #[← sortOf e.appArg!] }
    return { name := name.getString! }
  throwError "unsupported field sort {e}"

partial def normalizeSort (pos : Position) (sort : SortRef) : MetaM SortRef := do
  if sort.name ∈ ["Option", "List"] then
    unless sort.arguments.size == 1 do throwError pos.error "Option and List require one sort"
    return { sort with arguments := #[← normalizeSort pos sort.arguments[0]!] }
  unless sort.arguments.isEmpty do throwError pos.error "only Option and List accept sort arguments"
  let name := if sort.name ∈ ["Nat", "Int", "Bool", "String"] then sort.name.toName
    else `Semantics ++ sort.name.toName
  unless (← getEnv).contains name do throwError pos.error s!"unknown sort '{sort.name}'"
  sortOf (mkConst name)

structure Field where
  name : String
  sort : SortRef
  hasDefault : Bool := false
  deriving Inhabited, Repr

structure Signature where
  constant : Name
  fields : Array Field
  result : SortRef
  deriving Inhabited, Repr

def signature (name : Name) : MetaM Signature := do
  forallTelescope (← getConstInfo name).type fun args result => do
    let mut fields := #[]
    for arg in args do
      let decl ← arg.fvarId!.getDecl
      if decl.binderInfo.isExplicit then
        fields := fields.push {
          name := decl.userName.toString
          sort := (← sortOf decl.type)
          hasDefault := decl.type.getOptParamDefault?.isSome }
    return { constant := name, fields, result := (← sortOf result) }

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
  return result

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
