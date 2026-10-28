keyword buyback (cost : Cost) : Ability := {
  lexical := { lemma := "buyback", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): buyback [mana cost] / buyback—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [static (addedCost cost true Semantics.PayTimes.once),
      static
        (replace
          (causedBy
            (byEvent
              (does Subject.any
                (resolve thisSpell)))
            (would Subject.any
              (putInto thisSpell graveyard
                (some stack))))
          (doIf
            (keywordCostPaid buyback.label thisSpell)
            (instead
              (actionMove thisSpell wherever
                (some hand) Semantics.ObservationPoint.arrival []))))]
}
