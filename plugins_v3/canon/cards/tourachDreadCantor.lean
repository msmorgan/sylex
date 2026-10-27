/-- Tourach, Dread Cantor -/
card tourachDreadCantor : Spelled :=
  { name := "Tourach, Dread Cantor", cost := [generic 1, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ kicker [pip .black, pip .black],
        protection (hasColor .white),
        tourachDiscardTrigger,
        when (does thisCreature enter) (if_ := keywordCostPaid kicker
                                                         thisCreature)
          (instruct (target isOpponent) (discard (countedAtRandom (exactly 2) isCard))) ],
    power := stat 2, toughness := stat 1 }
