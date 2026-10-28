card := {
  name := "Ingenious Artillerist",
  cost := [2, red],
  types := [creature],
  subtypes := [human, artificer],
  text := [
    whenever (does (counted (orMore 1) [isArtifact, youControl]) enter)
      (instruct thisCreature (dealDamage groupSize eachOpponent))
  ],
  power := 3,
  toughness := 1
}
