/-- Soldevi Adnate -/
card soldeviAdnate : Spelled :=
  { name := "Soldevi Adnate", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ activated
          (compound [tapSymbol,
            (sacrifice (a (and [isCreature, or [hasColor .black, isArtifact]])))])
          (addMana (statOf (.stat .manaValue) (the (and [was (sacrifice which), isCreature])))
            (manaOf [[.of .black]])) ],
    power := stat 1, toughness := stat 2 }
