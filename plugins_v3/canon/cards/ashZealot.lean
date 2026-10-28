card := {
  name := "Ash Zealot",
  cost := [red, red],
  types := [creature],
  subtypes := [human, warrior],
  text := [
    firstStrike,
    haste,
    whenever
      (does (a isPlayer) (cast (a [isSpell, did any (cast which (from_ := graveyard)) thisObject])))
      (instruct thisCreature (dealDamage 3 (that player)))
  ],
  power := 2,
  toughness := 2
}
