/-- "[Quality] offering" means "As an additional cost to cast this spell, you may sacrifice a
[quality] permanent. If you chose to pay the additional cost, this spell's total cost is reduced
by the sacrificed permanent's mana cost, and you may cast this spell any time you could cast an
instant." [CR#702.48a] -/
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
