/-- "Replicate [cost]" means "As an additional cost to cast this spell, you may pay [cost] any
number of times" and "When you cast this spell, if a replicate cost was paid for it, copy it for
each time its replicate cost was paid. If the spell has any targets, you may choose new targets
for any of the copies." [CR#702.56a] -/
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
