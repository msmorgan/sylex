/-- "Buyback [cost]" means "You may pay an additional [cost] as you cast this spell" and "If the
buyback cost was paid, put this spell into its owner's hand instead of into that player's
graveyard as it resolves." [CR#702.27a] -/
keyword buyback (cost : Cost) : Ability := {
  lexical := { lemma := "buyback", parameters := [[Cost], [DashNonManaCost]] },
  spelling := "buyback <cost>",
  body :=
    [static (addedCost cost true .once),
      static
        (replace
          (causedBy
            (byEvent
              (does any
                (resolve thisSpell)))
            (would any
              (putInto thisSpell graveyard
                (some stack))))
          (doIf
            (keywordCostPaid buyback.label thisSpell)
            (instead
              (move thisSpell wherever
                (some hand) .arrival []))))]
}
