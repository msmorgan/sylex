import SylGen.Environment
import SylGen.BundleOrdering

open Lean Semantics.Authoring
namespace SylGen

private def identifierReferences (bound : Array String) (word : String) : List Name :=
  if bound.contains word then [] else
    let original := word.toName
    let name := original.getPrefix ++ Name.mkSimple (declarationWord original.getString!)
    if word == "this" then [`this, `this_]
    else if word ∈ ["creature", "artifact", "enchantment", "land", "planeswalker", "battle",
        "instant", "sorcery", "kindred", "card", "permanent", "player", "token", "spell"] then
      [name, Name.mkSimple ("is" ++ word.capitalize)]
    else [name, name.replacePrefix `Semantics.Templates .anonymous,
      name.replacePrefix `Templates .anonymous,
      name.replacePrefix `Semantics.Definitions .anonymous]

private partial def references (bound : Array String) : Value → List Name
  | .word _ name args => identifierReferences bound name ++
      args.toList.flatMap (fun a => references bound a.2)
  | .list _ values => values.toList.flatMap (references bound)
  | .record _ fields => fields.toList.flatMap (fun f => references bound f.2)
  | _ => []

private partial def sortReferences (bound : Array String) (sort : SortRef) : List Name :=
  identifierReferences bound sort.name ++ sort.arguments.toList.flatMap (sortReferences bound)

private partial def sortSyntax (sort : SortRef) : String :=
  if sort.arguments.isEmpty then sort.name else
    sort.name ++ " " ++ " ".intercalate (sort.arguments.toList.map fun arg =>
      if arg.arguments.isEmpty then sortSyntax arg else "(" ++ sortSyntax arg ++ ")")

private def bundleItem (d : Declaration) : BundleItem :=
  let refsFor := fun bound =>
    d.cardValue.toList.flatMap (references bound) ++
    (d.fields.toList.filter (fun f => !(f.1 ∈ ["lexical", "spelling", "ofAbility"]))).flatMap
      (fun f => references bound f.2) ++ sortReferences bound d.sort ++
      d.parameters.toList.flatMap (fun p => identifierReferences bound p.name ++
        sortReferences bound p.sort ++
        p.defaultValue.toList.flatMap (references bound))
  { key := { bareName := Name.mkSimple (declarationWord d.name)
             returnType := `Semantics ++ Name.mkSimple (sortSyntax d.sort) }
    file := d.pos.error ""
    references := refsFor (d.parameters.map (·.name)) ++
      (if d.kind == "ability_word" then [`abilityWord] else [])
    primitiveReferences := refsFor #[]
    primitive := d.kind == "primitive"
    valueRegistered := d.kind ∈ ["counter", "designation", "subtype", "card_type", "relation"] }

/-- Use the same dependency and ordering implementation as the Lean bundle loader. -/
def orderDeclarations (ds : Array Declaration) : Except String (Array Declaration) := do
  let items := ds.toList.map bundleItem
  let sorted ← topoSortBundle items
  let byKey := Std.HashMap.ofList (items.zip ds.toList |>.map fun (item, d) => (item.key, d))
  let catalog := sorted.filter (·.valueRegistered)
  let bodies := sorted.filter (! ·.valueRegistered)
  return (catalog ++ bodies).toArray.map (byKey[·.key]!)

end SylGen
