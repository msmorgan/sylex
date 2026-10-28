/-- "Rampage N" means "Whenever this creature becomes blocked, it gets +N/+N until end of turn for
each creature blocking it beyond the first." [CR#702.23a] -/
keyword rampage (amount : Amount) : Ability := {
  lexical := { lemma := "rampage", parameters := [[Amount]] },
  spelling := "rampage <amount>",
  body :=
    [whenever
        (does any
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
