card := {
  name := "Martyrs of Korlis",
  cost := [3, white, white],
  types := [creature],
  subtypes := [human],
  text := [
    onlyWhile
      (replace (would (by_ (allOf isArtifact)) (dealDamage anyAmount you))
         (instead (redirectDamage allDamage thisCreature)))
      (is thisCreature isUntapped)
  ],
  power := 1,
  toughness := 6
}
