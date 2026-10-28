card := {
  name := "Clarion Conqueror",
  cost := [2, white],
  types := [creature],
  subtypes := [dragon],
  text := [
    flying,
    cant any
      (activate
         (allOf
            [
              abilityHead anyActivated,
              abilityOf (allOf (or [isArtifact, isCreature, isPlaneswalker]))
            ]))
  ],
  power := 3,
  toughness := 3
}
