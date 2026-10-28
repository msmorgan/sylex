card := {
  name := "Pithing Needle",
  cost := [1],
  types := [artifact],
  text := [
    entersChoosing thisArtifact cardName,
    cant any
      (activate
         (allOf
            [
              abilityHead anyActivated,
              abilityOf (allOf [isSource, named chosenName]),
              not isManaAbility
            ]))
  ]
}
