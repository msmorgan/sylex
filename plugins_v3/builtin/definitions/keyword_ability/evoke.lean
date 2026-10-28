/-- "Evoke [cost]" means "You may cast this card by paying [cost] rather than paying its mana cost"
and "When this permanent enters, if its evoke cost was paid, its controller sacrifices it."
[CR#702.74a] -/
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
