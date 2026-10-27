card gloriousEnforcer : Spelled :=
  { name := "Glorious Enforcer", cost := [generic 5, pip .white, pip .white],
    types := [.creature], subtypes := [angel],
    text :=
      [ flying, lifelink,
        at_ (beginningOfPossessed .combat eachPlayer)
          (if_ := compareAmt (lifeTotalOf you) .greater (lifeTotalOf anOpponent))
          (gain thisCreature doubleStrike untilEndOfTurn) ],
    power := stat 5, toughness := stat 5 }
