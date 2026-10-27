keyword buyback (cost : Cost) : Ability := {
  lexical := { lemma := "buyback", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): buyback [mana cost] / buyback—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [Semantics.Templates.Ability.static (Semantics.Templates.StaticSpec.addedCost cost Bool.true Semantics.PayTimes.once),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.causedBy
            (Semantics.Templates.Causing.byEvent
              (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
                (Semantics.Definitions.resolve Semantics.Templates.NounPhrase.thisSpell)))
            (Semantics.Templates.WouldClause.would Semantics.Templates.Subject.any
              (Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.thisSpell Semantics.Templates.Location.graveyard
                (Option.some Semantics.Templates.Location.stack))))
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.buyback.label Semantics.Templates.NounPhrase.thisSpell)
            (Semantics.Templates.Replacement.instead
              (Semantics.Templates.Action.actionMove Semantics.Templates.NounPhrase.thisSpell Semantics.Templates.Location.wherever
                (Option.some Semantics.Templates.Location.hand) Semantics.ObservationPoint.arrival []))))]
}
