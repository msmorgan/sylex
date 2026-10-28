keyword squad (cost : Cost) : Ability := {
  lexical := { lemma := "squad", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): squad [mana cost] / squad—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [static (addedCost cost true Semantics.PayTimes.anyNumberOfTimes),
      when
        (does (actor thisCreature) enter)
        (create
          (timesPaid (Semantics.LinkSource.byKeyword squad.label)
            thisCreature)
          (copyOf thisCreature []))
        [] [] none none none
        (some
          (keywordCostPaid squad.label thisCreature))]
}
