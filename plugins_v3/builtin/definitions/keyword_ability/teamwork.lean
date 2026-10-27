keyword teamwork (amount : Amount) : Ability := {
  lexical := { lemma := "teamwork", parameters := [[Amount]] },
  spelling := "teamwork <amount>",
  body :=
    [ static
        (.addedCost
          (.perform
            (tap
              (withTotal (.stat .power) .atLeast amount
                (allOf (and [isCreature, youControl])))))
          true .once) ]
}
