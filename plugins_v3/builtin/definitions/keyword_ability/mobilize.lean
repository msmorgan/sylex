/-- "Mobilize N" means "Whenever this creature attacks, create N 1/1 red Warrior creature tokens.
Those tokens enter tapped and attacking. Sacrifice them at the beginning of the next end step."
[CR#702.181a] -/
keyword mobilize (amount : Amount) : Ability := {
  lexical := { lemma := "mobilize", parameters := [[Amount]] },
  spelling := "mobilize <amount>",
  body :=
    [whenever
        (does (actor thisCreature)
          (attack none))
        (sequentially
          [createTappedAttacking amount
              (creatureToken 1 1 [.red] [warrior]),
            atNext .endStep
              (sacrifice thoseTokens)])]
}
