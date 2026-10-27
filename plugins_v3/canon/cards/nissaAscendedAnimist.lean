/-- Nissa, Ascended Animist -/
card nissaAscendedAnimist : Spelled :=
  { name := "Nissa, Ascended Animist",
    cost := [generic 3, pip .green, pip .green, phyrexianPip .green, phyrexianPip .green],
    supertypes := [.legendary], types := [.planeswalker], subtypes := [nissa],
    text :=
      [ compleated,
        activated (loyaltySymbol (up 1))
          (sequentially
            [ create 1 (creatureTokenOf (letter .x) (letter .x) [.green] [phyrexian, horror]),
              define .x (statOf (.stat .loyalty) thisPlaneswalker) ]),
        activated (loyaltySymbol (down 1)) (destroy (target (or [isArtifact, isEnchantment]))),
        activated (loyaltySymbol (down 7))
          (establishFor (allOf (and [isCreature, youControl]))
            [ getsPt them (up (forEach 1 (and [forest, youControl])))
                (up (forEach 1 (and [forest, youControl]))),
              abilityGrant them trample ]
            untilEndOfTurn) ],
    loyalty := stat 7 }
