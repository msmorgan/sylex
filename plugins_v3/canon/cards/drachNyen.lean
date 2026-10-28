card := {
  name := "Drach'Nyen",
  cost := [4, black, red],
  supertypes := [legendary],
  types := [artifact],
  subtypes := [equipment],
  text := [
    when (does thisEquipment enter) (exile (targets (upTo 1) isCreature)),
    static
      [
        abilityGrant (equipped creature) menace,
        getsPt (equipped creature) (up (letter x)) (up 0),
        letterDefinition x (powerOf (the (in_ (exiledWith thisEquipment))))
      ],
    equip [2]
  ]
}
