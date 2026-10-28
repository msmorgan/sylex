/-- Juxtapose: "You and target player exchange control of the creature you each control with the
greatest mana value. Then exchange control of artifacts the same way. If two or more permanents
a player controls are tied for greatest, their controller chooses one of them." "The same way"
is written out as the second exchange. -/
card := {
  name := "Juxtapose",
  cost := [3, blue],
  types := [sorcery],
  text := [
    spell
      [
        exchangeControl
          (the
             [
               isCreature,
               controlledBy you,
               superlative max (stat manaValue) [isCreature, controlledBy you]
             ])
          (the
             [
               isCreature,
               controlledBy (target isPlayer),
               superlative max (stat manaValue) [isCreature, controlledBy (target isPlayer)]
             ]),
        exchangeControl
          (the
             [
               isArtifact,
               controlledBy you,
               superlative max (stat manaValue) [isArtifact, controlledBy you]
             ])
          (the
             [
               isArtifact,
               controlledBy (that player),
               superlative max (stat manaValue) [isArtifact, controlledBy (that player)]
             ]),
        (doIf
           (thereAre
              (counted (orMore 2)
                 [
                   isPermanent,
                   controlledBy (a isPlayer),
                   tiedFor max (stat manaValue) [isPermanent, controlledBy (that player)]
                 ]))
           (instruct (controllerOf them) (choose (someOf (exactly 1) them))))
      ]
  ]
}
