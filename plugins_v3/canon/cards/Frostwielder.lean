card := {
  name := "Frostwielder",
  cost := [2, red, red],
  types := [creature],
  subtypes := [human, shaman],
  text := [
    replace
      (would (a [isCreature, did thisCreature (dealDamage anyAmount which) (within soFarThisTurn)])
         die)
      (instead (exile it)),
    activated tapSymbol (instruct thisCreature (dealDamage 1 (target isAnyTarget)))
  ],
  power := 1,
  toughness := 2
}
