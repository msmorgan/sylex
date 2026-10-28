/-- Extort is a triggered ability. “Extort” means “Whenever you cast a spell, you may pay {W/B}. If
you do, each opponent loses 1 life and you gain life equal to the total life lost this way.”
[CR#702.101a] -/
keyword extort : Ability := {
  lexical := { lemma := "extort" },
  spelling := "extort",
  body :=
    [whenever
        (does (actor you)
          (cast (a isSpell)))
        (may
          (pay
            (mana [hybridPip white black])
            once)
          (some
            (sequentially
              [instruct eachOpponent (loseLife 1),
                gainLife thatMuch])))]
}
