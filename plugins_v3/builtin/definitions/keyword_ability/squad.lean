keyword squad (cost : Cost) : Ability := {
  lexical := { lemma := "squad", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): squad [mana cost] / squad—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [Semantics.Templates.Ability.static (Semantics.Templates.StaticSpec.addedCost cost Bool.true Semantics.PayTimes.anyNumberOfTimes),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature) Semantics.Templates.Action.enter)
        (Semantics.Definitions.create
          (Semantics.Templates.Amount.timesPaid (Semantics.LinkSource.byKeyword Semantics.Definitions.squad.label)
            Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.TokenSpec.copyOf Semantics.Templates.NounPhrase.thisCreature []))
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.squad.label Semantics.Templates.NounPhrase.thisCreature))]
}
