keyword evoke (cost : Cost) : Ability := {
  lexical := { lemma := "evoke", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): evoke [mana cost] / evoke—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [ static (altCost this_ (some cost)),
      when (does thisPermanent enter)
        (if_ := keywordCostPaid evoke thisPermanent)
        (instruct (controllerOf thisPermanent) (sacrifice thisPermanent)) ]
}
