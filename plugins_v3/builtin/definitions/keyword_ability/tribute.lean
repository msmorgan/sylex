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
