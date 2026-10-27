keyword buyback (cost : Cost) : Ability := {
  lexical := { lemma := "buyback", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): buyback [mana cost] / buyback—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [ static (.addedCost cost true .once),
      static
        (replace
          (.causedBy (.event (does .any (resolve thisSpell)))
            (.would .any (putInto thisSpell graveyard (from_ := stack))))
          (doIf (keywordCostPaid buyback thisSpell)
            (instead (.move thisSpell .anyLocation hand .arrival [])))) ]
}
