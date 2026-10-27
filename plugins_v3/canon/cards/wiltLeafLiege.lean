/-- Wilt-Leaf Liege -/
card wiltLeafLiege : Spelled :=
  { name := "Wilt-Leaf Liege",
    cost := [generic 1, hybridPip .green .white, hybridPip .green .white, hybridPip .green .white],
    types := [.creature], subtypes := [elf, knight],
    text :=
      [ static (getsPt (allOf (and [isCreature, hasColor .green, youControl, isOther]))
          (up 1) (up 1)),
        static (getsPt (allOf (and [isCreature, hasColor .white, youControl, isOther]))
          (up 1) (up 1)),
        (static (replace
              (causedBy
                (bySource (a (and [or [isSpell, abilityHead .anyOnStack], anOpponentControls])))
                (would you (discard thisCard)))
              (instead (putOntoBattlefield it)))) ],
    power := stat 4, toughness := stat 4 }
