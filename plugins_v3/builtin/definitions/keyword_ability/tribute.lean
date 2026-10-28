/-- "Tribute N" means "As this creature enters, choose an opponent. That player may put an
additional N +1/+1 counters on it as it enters." [CR#702.104a] -/
keyword tribute (amount : Amount) : Ability := {
  lexical := {
    lemma := "tribute",
    parameters := [[Amount]],
    nouns := [{ lemma := "tribute", countability := [Mass] }]
  },
  spelling := "tribute <amount>",
  body :=
    [ static
        (replace (would thisCreature enter)
          (before (choose anOpponent)
            (some
              (alongside
                (instruct (the isChosenPlayer)
                  (may (putCounters amount p1p1Counter thisCreature))))))) ]
}
