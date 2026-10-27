/-- Juxtapose: "You and target player exchange control of the creature you each control with the
greatest mana value. Then exchange control of artifacts the same way. If two or more permanents
a player controls are tied for greatest, their controller chooses one of them." "The same way"
is written out as the second exchange. -/
card juxtapose : Spelled :=
  { name := "Juxtapose", cost := [generic 3, pip .blue], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ exchangeControl ((the (and [isCreature, controlledBy you,
                        superlative .max (.stat .manaValue) (and [isCreature, controlledBy you])])))
              ((the (and [isCreature, controlledBy (target isPlayer),
                            superlative .max (.stat .manaValue) (and [isCreature, controlledBy
                              (target isPlayer)])]))),
            exchangeControl ((the (and [isArtifact, controlledBy you,
                          superlative .max (.stat .manaValue) (and [isArtifact, controlledBy
                            you])])))
              ((the (and [isArtifact, controlledBy (that player),
                            superlative .max (.stat .manaValue) (and [isArtifact, controlledBy (that
                              player)])]))),
            (doIf (thereAre (counted (orMore 2)
                        (and [isPermanent, controlledBy (a isPlayer),
                              tiedFor .max (.stat .manaValue) (and [isPermanent, controlledBy (that
                                player)])])))
                  (instruct (controllerOf them) (choose (someOf (exactly 1) them)))) ]) ] }
