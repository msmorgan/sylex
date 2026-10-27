keyword replicate (cost : Cost) : Ability := {
  lexical := { lemma := "replicate", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): replicate [mana cost] / replicate—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
  body :=
    [Semantics.Templates.Ability.static (Semantics.Templates.StaticSpec.addedCost cost Bool.true Semantics.PayTimes.anyNumberOfTimes),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisSpell))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.copyOnStack Semantics.Templates.NounPhrase.thisSpell
              (Semantics.Templates.Amount.timesPaid (Semantics.LinkSource.byKeyword Semantics.Definitions.replicate.label)
                Semantics.Templates.NounPhrase.thisSpell),
            Semantics.Templates.Action.may
              (Semantics.Templates.Action.chooseNewTargets (Semantics.Templates.NounPhrase.those Semantics.NounWord.copy))])
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.replicate.label Semantics.Templates.NounPhrase.thisSpell))]
}
