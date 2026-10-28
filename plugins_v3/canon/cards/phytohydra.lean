card := {
  name := "Phytohydra",
  cost := [2, green, white, white],
  types := [creature],
  subtypes := [plant, hydra],
  text := [
    replace (would any (dealDamage anyAmount thisCreature))
      (instead (putCounters thatMuch p1p1Counter it))
  ],
  power := 1,
  toughness := 1
}
