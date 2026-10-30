import Lean

open Lean
namespace Semantics.Authoring

/-- A declaration's bare spelling and semantic result sort identify a bundle node. -/
structure BundleKey where
  bareName : Name
  returnType : Name
  deriving BEq, Hashable, Inhabited, Repr

structure BundleItem where
  key : BundleKey
  file : String
  references : List Name := []
  primitiveReferences : List Name := []
  valueRegistered : Bool := false
  primitive : Bool := false
  stx : Syntax := .missing
  deriving Inhabited

private def bundleKeyLess (a b : BundleKey) : Bool :=
  a.bareName.toString < b.bareName.toString ||
    (a.bareName == b.bareName && a.returnType.toString < b.returnType.toString)

def bundleDependencies (items : List BundleItem) (item : BundleItem) : List BundleKey :=
  if item.primitive then [] else
    let refSet := Std.HashSet.ofList item.references
    let primRefSet := Std.HashSet.ofList (item.references ++ item.primitiveReferences)
    items.filterMap fun other =>
      let qualified := Name.mkSimple other.key.returnType.getString! ++ other.key.bareName
      let row := Name.mkSimple (other.key.bareName.getString! ++ "_definition")
      let references := if other.primitive then primRefSet else refSet
      if ((other.valueRegistered && references.contains row) ||
          (!other.valueRegistered &&
            (references.contains other.key.bareName || references.contains qualified))) &&
        other.key != item.key &&
        (other.key.bareName != item.key.bareName || other.primitive ||
          (item.key.returnType == `Semantics.Ability && other.key.returnType == `Semantics.Action))
      then some other.key else none

/-- Kahn's algorithm, choosing the least ready (bare name, return type) at every step. -/
def topoSortBundle (items : List BundleItem) : Except String (List BundleItem) := Id.run do
  let mut seen : List BundleKey := []
  for item in items do
    if seen.contains item.key then
      return .error (s!"Duplicate bundle declaration: {item.key.bareName} : " ++
        s!"{item.key.returnType} ({item.file})")
    seen := item.key :: seen
  let mut byKey : Std.HashMap BundleKey BundleItem := {}
  let mut degrees : Std.HashMap BundleKey Nat := {}
  let mut dependents : Std.HashMap BundleKey (List BundleKey) := {}
  let mut ready := []
  for item in items do
    byKey := byKey.insert item.key item
    let deps := bundleDependencies items item
    degrees := degrees.insert item.key deps.length
    if deps.isEmpty then ready := item.key :: ready
    for dep in deps do
      dependents := dependents.insert dep (item.key :: dependents.getD dep [])
  ready := ready.mergeSort bundleKeyLess
  let mut sorted := []
  while !ready.isEmpty do
    let key := ready.head!
    ready := ready.tail!
    sorted := sorted ++ [byKey.getD key default]
    for dependent in dependents.getD key [] do
      let degree := degrees.getD dependent 0 - 1
      degrees := degrees.insert dependent degree
      if degree == 0 then ready := (dependent :: ready).mergeSort bundleKeyLess
  if sorted.length != items.length then
    let remaining := items.filter (fun item => degrees.getD item.key 0 > 0)
      |>.mergeSort (fun a b => bundleKeyLess a.key b.key)
    let rec reaches (fuel : Nat) (target : BundleKey) (pending seen : List BundleKey) : Bool :=
      match fuel, pending with
      | 0, _ | _, [] => false
      | fuel + 1, key :: rest =>
        if key == target then true
        else if seen.contains key then reaches fuel target rest seen
        else reaches fuel target
          (bundleDependencies items (byKey.getD key default) ++ rest) (key :: seen)
    let cyclic := remaining.filter fun item => reaches (items.length * (items.length + 1) + 1)
      item.key (bundleDependencies items item) []
    let summary := cyclic.map fun item =>
      s!"{item.key.bareName} : {item.key.returnType} ({item.file})"
    return .error s!"Dependency cycle in bundle: {", ".intercalate summary}"
  return .ok sorted

end Semantics.Authoring
