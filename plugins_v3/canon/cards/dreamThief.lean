card dreamThief : Spelled :=
  { name := "Dream Thief", cost := [generic 2, pip .blue], types := [.creature],
    subtypes := [faerie, rogue],
    text :=
      [ flying,
        when (does thisCreature enter)
          (doOnlyIf (draw 1)
            (did you (cast (a
              (and [isSpell, hasColor .blue,
              isOther]))) soFarThisTurn)) ],
    power := stat 2, toughness := stat 1 }
