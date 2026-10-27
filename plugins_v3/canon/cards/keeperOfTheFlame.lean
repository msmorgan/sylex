/-- Keeper of the Flame -/
card keeperOfTheFlame : Spelled :=
  { name := "Keeper of the Flame", cost := [pip .red, pip .red], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ activated (compound [[pip .red], tapSymbol])
          (sequentially
            [ chooseWhile
                (target (and [isOpponent, compare .greater (lifeTotalOf you) (axes := [.playerStat .lifeTotal])]))
                (whileDoing (does you (activate thisAbility))),
              instruct thisCreature (dealDamage 2 (that player)) ]) ],
    power := stat 1, toughness := stat 2 }
