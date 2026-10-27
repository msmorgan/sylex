keyword replicate (cost : Cost) : Ability := {
  lexical := { lemma := "replicate", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): replicate [mana cost] / replicate—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
  body :=
    [ static (.addedCost cost true .anyNumberOfTimes),
      when (does you (cast thisSpell))
        (if_ := keywordCostPaid replicate thisSpell)
        (sequentially
          [ copyOnStack thisSpell (timesPaid (.byKeyword replicate) thisSpell),
            may (.chooseNewTargets (those .copy)) ]) ]
}
