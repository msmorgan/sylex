keyword mobilize (amount : Amount) : Ability := {
  lexical := { lemma := "mobilize", parameters := [[Amount]] },
  spelling := "mobilize <amount>",
  body :=
    [ whenever (does thisCreature (.attack none))
        (sequentially
          [ createTappedAttacking amount (creatureToken 1 1 [.red] [warrior]),
            atNext .endStep
              (sacrifice thoseTokens) ]) ]
}
