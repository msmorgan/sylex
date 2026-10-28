/-- Enormous Energy Blade -/
card := {
  name := "Enormous Energy Blade",
  cost := [2, black],
  types := [artifact],
  subtypes := [equipment],
  text := [
    getsPt (equipped creature) (up 4) (up 0),
    whenever (does any (attachment attached thisEquipment (a isCreature))) (tap (that creature)),
    equip [2]
  ]
}
