/-- Ghostfire Blade -/
card := {
  name := "Ghostfire Blade",
  cost := [1],
  types := [artifact],
  subtypes := [equipment],
  text := [
    getsPt (equipped creature) (up 2) (up 2),
    equip [3],
    costShift
      (allOf
         [
           abilityHead (keyword equip),
           abilityOf this_,
           targeting (a [isCreature, isColorless]) someTarget
         ])
      (less 2)
  ]
}
