/-- Death or Glory -/
card deathOrGlory : Spelled :=
  { name := "Death or Glory", cost := [generic 4, pip .white], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ separateIntoPiles
              (allOf (and [isCreature, in_ yourGraveyard])) 2,
            exile (pileOfChoice anOpponent),
            returnToBattlefield theOther ]) ] }
