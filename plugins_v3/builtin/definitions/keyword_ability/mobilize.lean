keyword mobilize (amount : Amount) : Ability := {
  lexical := { lemma := "mobilize", parameters := [[Amount]] },
  spelling := "mobilize <amount>",
  body :=
    [whenever
        (does (actor thisCreature)
          (attack none))
        (sequentially
          [createTappedAttacking amount
              (creatureToken 1 1 [Semantics.Color.red] [warrior]),
            atNext Semantics.TurnPart.endStep
              (sacrifice thoseTokens)])]
}
