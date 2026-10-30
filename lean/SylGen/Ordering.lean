import SylGen.Environment

namespace SylGen

private partial def references (bound : Array String) : Value → Array String
  | .word _ name args =>
    (if bound.contains name then #[] else
      let mapped := leanWord name
      if name ∈ ["creature", "artifact", "enchantment", "land", "planeswalker", "battle",
          "instant", "sorcery", "kindred", "card", "permanent", "player", "token", "spell"] then
        #[mapped, "is" ++ name.capitalize]
      else #[mapped]) ++
      args.flatMap (fun a => references bound a.2)
  | .list _ values => values.flatMap (references bound)
  | .record _ fields => fields.flatMap (fun f => references bound f.2)
  | _ => #[]

private def catalog (d : Declaration) : Bool :=
  d.kind ∈ ["counter", "designation", "subtype", "card_type", "relation"]

private def less (a b : Declaration) : Bool :=
  leanWord a.name < leanWord b.name ||
    (leanWord a.name == leanWord b.name && a.sort.text < b.sort.text)

/-- Preserve the bundle loader's least-ready ordering for name allocation. SCC emission uses
its separately resolved graph; label cycles do not impose an order on their bodies. -/
def orderDeclarations (ds : Array Declaration) : Array Declaration := Id.run do
  let refsFor := fun (d : Declaration) (bound : Array String) =>
    d.cardValue.toArray.flatMap (references bound) ++
    (d.fields.filter (fun f => !(f.1 ∈ ["lexical", "spelling", "ofAbility"]))).flatMap
      (fun f => references bound f.2) ++
      d.parameters.flatMap (fun p => p.defaultValue.toArray.flatMap (references bound)) ++
      (if d.kind == "ability_word" then #["abilityWord"] else #[])
  let refs := ds.map fun d => Std.HashSet.ofArray (refsFor d (d.parameters.map (·.name)))
  let primitiveRefs := ds.map fun d => Std.HashSet.ofArray (refsFor d #[])
  let mut degrees := Array.replicate ds.size 0
  let mut dependents : Array (Array Nat) := Array.replicate ds.size #[]
  for i in [:ds.size] do
    unless ds[i]!.kind == "primitive" do
      for j in [:ds.size] do
        let names := if ds[j]!.kind == "primitive" then primitiveRefs[i]! else refs[i]!
        let referenced := if catalog ds[j]! then
          names.contains (leanWord ds[j]!.name ++ "_definition")
          else names.contains (leanWord ds[j]!.name)
        if i != j && referenced &&
            (ds[j]!.name != ds[i]!.name || ds[j]!.kind == "primitive" ||
              (ds[i]!.sort.name == "Ability" && ds[j]!.sort.name == "Action")) then
          degrees := degrees.modify i (· + 1)
          dependents := dependents.modify j (·.push i)
  let mut ready := (Array.range ds.size).filter (degrees[·]! == 0)
  let mut emitted := Array.replicate ds.size false
  let mut out := #[]
  while out.size < ds.size do
    -- Mutually referring keyword labels are emitted before their SCC's bodies.
    if ready.isEmpty then
      ready := ((Array.range ds.size).filter (!emitted[·]!)).qsort
        (fun i j => less ds[i]! ds[j]!) |>.extract 0 1
    ready := ready.qsort (fun i j => less ds[i]! ds[j]!)
    let next := ready[0]!
    ready := ready.extract 1 ready.size
    if emitted[next]! then continue
    emitted := emitted.set! next true
    out := out.push ds[next]!
    for dependent in dependents[next]! do
      degrees := degrees.modify dependent (· - 1)
      if degrees[dependent]! == 0 && !emitted[dependent]! then ready := ready.push dependent
  return (out.filter catalog) ++ (out.filter (!catalog ·))

end SylGen
