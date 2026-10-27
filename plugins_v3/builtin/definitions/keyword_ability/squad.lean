keyword squad (cost : Cost) : Ability := {
  lexical := { lemma := "squad", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): squad [mana cost] / squad—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [ static (.addedCost cost true .anyNumberOfTimes),
      when (does thisCreature enter)
        (if_ := keywordCostPaid squad thisCreature)
        (create (timesPaid (.byKeyword squad) thisCreature) (.copyOf thisCreature [])) ]
}
