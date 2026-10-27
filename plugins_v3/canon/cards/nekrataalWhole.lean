card nekrataalWhole : Spelled :=
  { name := "Nekrataal", cost := [generic 2, pip .black, pip .black], types := [.creature],
    subtypes := [human, assassin],
    text :=
      [ firstStrike,
        when (does thisCreature enter)
          (doUnderProhibition (destroy (target (and [isCreature, non isArtifact, not (hasColor
              .black)])))
            (regeneration (that creature))) ],
    power := stat 2, toughness := stat 1 }
