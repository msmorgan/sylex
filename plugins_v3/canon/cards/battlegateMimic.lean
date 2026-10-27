/-- Battlegate Mimic -/
card battlegateMimic : Spelled :=
  { name := "Battlegate Mimic", cost := [generic 1, hybridPip .red .white], types := [.creature],
    subtypes := [shapeshifter],
    text :=
      [ whenever (does you (cast (a
        (and [isSpell, hasColor .red,
        hasColor .white]))))
          (establishThroughout (conjunction [ getsBase thisCreature 4 2,
                abilityGrant thisCreature firstStrike ]) untilEndOfTurn) ],
    power := stat 2, toughness := stat 1 }
