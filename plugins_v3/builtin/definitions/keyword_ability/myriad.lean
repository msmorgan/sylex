keyword myriad : Ability := {
  lexical := { lemma := "myriad" },
  spelling := "myriad",
  body :=
    [whenever
        (does (actor thisCreature)
          (attack none))
        (sequentially
          [doForEach
              (each
                (and
                  [isOpponent, otherThan defendingPlayer]))
              (may
                (create 1 (copyOf thisCreature)
                  [entersAs .tapped,
                    entersAttacking
                      (some
                        (or
                          [they,
                            a
                              (and
                                [isPlaneswalker,
                                  controlledBy they])]))])),
            doIf
              (compareAmt (numberOf thoseTokens)
                .atLeast 1)
              (atNext .endOfCombat
                (exile thoseTokens))])]
}
