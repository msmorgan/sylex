card brightspearZealot : Spelled :=
  { name := "Brightspear Zealot", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [human, soldier],
    text :=
      [ vigilance,
        static (onlyWhile (getsPt thisCreature (up 2) (up 0))
          (compareAmt (eventCount (cast (a isSpell)) you soFarThisTurn) .atLeast 2)) ],
    power := stat 2, toughness := stat 4 }
