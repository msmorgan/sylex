card := {
  name := "Sengir Vampire",
  cost := [3, black, black],
  types := [creature],
  subtypes := [vampire],
  text := [
    flying,
    whenever
      (does (a [isCreature, did thisCreature (dealDamage anyAmount which) (within soFarThisTurn)])
         die)
      (putCounters 1 p1p1Counter thisCreature)
  ],
  power := 4,
  toughness := 4
}
