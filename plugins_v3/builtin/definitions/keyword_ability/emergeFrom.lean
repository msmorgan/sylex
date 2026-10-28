/-- "Emerge from [quality] [cost]", a variant of emerge, so it carries emerge's label, its
quality telling it apart, and its cost is an emerge cost [CR#702.119b]. -/
keyword emergeFrom (quality : Predicate) (cost : Cost) : Ability := {
  lexical := { lemma := "emerge", parameters := [[Cost], [Quality (Some ("from")), Cost]] },
  label := "emerge",
  spelling := "emerge from <quality> <cost>",
  body :=
    [static
        (altCost this_
          (some
            (compound
              [cost,
                perform
                  (sacrifice
                    (a
                      (and [quality, isPermanent])))]))),
      static
        (onlyWhile
          (costShift thisSpell
            (less
              (statOf (Semantics.ProjAxis.stat Semantics.Stat.manaValue)
                (the
                  (and
                    [was (sacrifice which),
                      isPermanent])))))
          (keywordCostPaid emerge.label thisSpell))]
}
