card := {
  name := "Sanctuary Blade",
  cost := [2],
  types := [artifact],
  subtypes := [equipment],
  text := [
    attachChoosing thisEquipment color,
    static
      [
        getsPt (equipped creature) (up 2) (up 0),
        abilityGrant (equipped creature) (protection (ofTheLastChosen color))
      ],
    equip [3]
  ]
}
