/-- "Squad [cost]" means "As an additional cost to cast this spell, you may pay [cost] any number of
times" and "When this creature enters, if its squad cost was paid, create a token that's a copy
of it for each time its squad cost was paid." [CR#702.157a] -/
keyword squad (cost : Cost) : Ability := {
  lexical := { lemma := "squad", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): squad [mana cost] / squad—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [static (addedCost cost true .anyNumberOfTimes),
      when
        (does (actor thisCreature) enter)
        (create
          (timesPaid (.byKeyword squad.label)
            thisCreature)
          (copyOf thisCreature []))
        [] [] none none none
        (some
          (keywordCostPaid squad.label thisCreature))]
}
