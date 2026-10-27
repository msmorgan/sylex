/-- Custodi Squire -/
card custodiSquire : Spelled :=
  { name := "Custodi Squire", cost := [generic 4, pip .white], types := [.creature],
    subtypes := [spirit, cleric],
    text :=
      [ flying,
        willOfTheCouncil
          (when (does thisCreature enter)
            (sequentially
              [ instruct eachPlayer (voteStartingWith you .openly
                  (byCandidate
                    (a (and [ or [isArtifact, isCreature, isEnchantment],
                               in_ yourGraveyard ])))),
                returnToHand (allOf (and [isCard, withMostVotes])) ])) ],
    power := stat 3, toughness := stat 3 }
