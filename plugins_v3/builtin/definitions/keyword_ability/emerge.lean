keyword emerge (cost : Cost) : Ability := {
  lexical := { lemma := "emerge", parameters := [[Cost], [Quality (Some ("from")), Cost]] },
  spelling := "emerge <cost>",
  body :=
    [static
        (altCost this_
          (some
            (compound
              [cost,
                perform
                  (sacrifice (a isCreature))]))),
      static
        (onlyWhile
          (costShift thisSpell
            (less
              (statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue)
                (the
                  (and
                    [was (sacrifice which),
                      isCreature])))))
          (keywordCostPaid emerge.label thisSpell))]
}
