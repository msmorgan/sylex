card := {
  name := "Old-Growth Dryads",
  cost := [green],
  types := [creature],
  subtypes := [dryad],
  text := [
    when (does thisCreature enter)
      (instruct eachOpponent
         (may
            [
              search (libraryOf they) (a [isLand, hasSupertype basic]),
              putOntoBattlefieldTapped it,
              shuffle
            ]))
  ],
  power := 3,
  toughness := 3
}
