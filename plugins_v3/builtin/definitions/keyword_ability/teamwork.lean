keyword teamwork (amount : Amount) : Ability := {
  lexical := { lemma := "teamwork", parameters := [[Amount]] },
  spelling := "teamwork <amount>",
  body :=
    [static
        (addedCost
          (perform
            (tap
              (withTotal (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast
                amount
                (allOf
                  (and [isCreature, youControl])))))
          true Semantics.PayTimes.once)]
}
