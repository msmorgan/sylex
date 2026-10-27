keyword rampage (amount : Amount) : Ability := {
  lexical := { lemma := "rampage", parameters := [[Amount]] },
  spelling := "rampage <amount>",
  body :=
    [ whenever (does any (becomeBlocked thisCreature))
        (get thisCreature
          (.up
            (times amount
              (minus (countOf (and [isCreature, .inCombat .blockerOf thisCreature]))
                1)))
          (.up
            (times amount
              (minus (countOf (and [isCreature, .inCombat .blockerOf thisCreature]))
                1)))
          untilEndOfTurn) ]
}
