keyword replicate (cost : Cost) : Ability := {
  lexical := { lemma := "replicate", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): replicate [mana cost] / replicate—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
  body :=
    [static (addedCost cost true .anyNumberOfTimes),
      when
        (does (actor you)
          (cast thisSpell))
        (sequentially
          [copyOnStack thisSpell
              (timesPaid (.byKeyword replicate.label)
                thisSpell),
            may
              (chooseNewTargets (those .copy))])
        [] [] none none none
        (some
          (keywordCostPaid replicate.label thisSpell))]
}
