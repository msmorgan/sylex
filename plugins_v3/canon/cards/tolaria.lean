/-- Tolaria -/
card tolaria : Spelled :=
  { name := "Tolaria", supertypes := [.legendary], types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.of .blue]])),
        activatedOnlyDuring (compound [tapSymbol])
          (establishThroughout (abilityLoss (target isCreature)
              [written banding, abilities bandsWithOtherAbilities]) untilEndOfTurn)
          (duringPart .upkeep none) ] }
