/-- Ancestral Blade -/
card := {
  name := "Ancestral Blade",
  cost := [1, white],
  types := [artifact],
  subtypes := [equipment],
  text := [
    when (does thisEquipment enter)
      [create 1 (creatureToken 1 1 [white] [soldier]), attachTo thisEquipment thatToken],
    getsPt (equipped creature) (up 1) (up 1),
    equip [1]
  ]
}
