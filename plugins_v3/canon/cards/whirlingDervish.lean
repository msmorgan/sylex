/-- Whirling Dervish -/
card whirlingDervish : Spelled :=
  { name := "Whirling Dervish", cost := [pip .green, pip .green], types := [.creature],
    subtypes := [human, monk],
    text :=
      [ protection (hasColor .black),
        at_ (beginningOfPossessed .endStep eachPlayer)
          (if_ := some (did thisCreature (dealDamage anyAmount anOpponent) soFarThisTurn))
          (putCounters 1 p1p1Counter it) ],
    power := stat 1, toughness := stat 1 }
