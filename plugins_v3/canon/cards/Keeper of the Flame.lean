card := {
  name := "Keeper of the Flame",
  cost := [red, red],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    activated (compound [[red], tapSymbol])
      [
        chooseWhile
          (target [isOpponent, compare greater (lifeTotalOf you) (axes := [playerStat lifeTotal])])
          (whileDoing (does you (activate thisAbility))),
        instruct thisCreature (dealDamage 2 (that player))
      ]
  ],
  power := 1,
  toughness := 2
}
