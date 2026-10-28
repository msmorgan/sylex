keyword offering (quality : Predicate) : Ability := {
  lexical := { lemma := "offering", parameters := [[QualityBefore]] },
  spelling := "<quality> offering",
  body :=
    [static
        (addedCost
          (perform
            (sacrifice
              (a (and [quality, isPermanent]))))
          true .once),
      static
        (onlyWhile
          (reducedBySymbols thisSpell
            (manaCostOf
              (the
                (and
                  [was (sacrifice which),
                    isPermanent]))))
          (keywordCostPaid offering.label thisSpell)),
      static
        (onlyWhile
          (can (actor you)
            (cast this_) (some asInstant))
          (keywordCostPaid offering.label this_))]
}
