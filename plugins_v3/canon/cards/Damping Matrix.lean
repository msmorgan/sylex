card := {
  name := "Damping Matrix",
  cost := [3],
  types := [artifact],
  text := [
    cant any
      (activate
         (allOf
            [
              abilityHead anyActivated,
              abilityOf (allOf (or [isArtifact, isCreature])),
              not isManaAbility
            ]))
  ]
}
