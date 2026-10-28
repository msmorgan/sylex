keyword rampage (amount : Amount) : Ability := {
  lexical := { lemma := "rampage", parameters := [[Amount]] },
  spelling := "rampage <amount>",
  body :=
    [whenever
        (does Templates.Subject.any
          (becomeBlocked thisCreature))
        (get thisCreature
          (up
            (times amount
              (minus
                (countOf
                  (and
                    [isCreature,
                      inCombat .blockerOf
                        (some thisCreature)]))
                1)))
          (up
            (times amount
              (minus
                (countOf
                  (and
                    [isCreature,
                      inCombat .blockerOf
                        (some thisCreature)]))
                1)))
          (some untilEndOfTurn))]
}
