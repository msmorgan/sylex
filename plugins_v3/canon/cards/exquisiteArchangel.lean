card := {
  name := "Exquisite Archangel",
  cost := [5, white, white],
  types := [creature],
  subtypes := [angel],
  text := [
    flying,
    replace (would you loseGame)
      (instead [exile thisCreature, setLife (statOf (playerStat startingLifeTotal) you)])
  ],
  power := 5,
  toughness := 5
}
