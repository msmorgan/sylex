/-- Ahn-Crop Invader -/
card ahnCropInvader : Spelled :=
  { name := "Ahn-Crop Invader", cost := [generic 2, pip .red], types := [.creature],
    subtypes := [zombie, minotaur, warrior],
    text :=
      [ static (partScope .turn you (abilityGrant thisCreature firstStrike)),
        activated (compound [[generic 1], (sacrifice (a (and [isCreature,
            isOther])))])
          (get thisCreature (up 2) (up 0) untilEndOfTurn) ],
    power := stat 2, toughness := stat 2 }
