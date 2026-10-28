card := {
  name := "Chainsaw",
  cost := [1, red],
  types := [artifact],
  subtypes := [equipment],
  text := [
    when (does thisEquipment enter) (instruct it (dealDamage 3 (targets (upTo 1) isCreature))),
    whenever (does (counted (orMore 1) isCreature) die) (putCounters 1 revCounter thisEquipment),
    static
      [
        getsPt (equipped creature) (up (letter x)) (up 0),
        letterDefinition x (countersOn revCounter thisEquipment)
      ],
    equip [3]
  ]
}
