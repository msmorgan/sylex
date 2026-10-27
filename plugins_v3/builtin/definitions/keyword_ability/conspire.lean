keyword conspire : Ability := {
  lexical := { lemma := "conspire" },
  spelling := "conspire",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.addedCost
          (Semantics.Templates.Cost.perform
            (Semantics.Definitions.tap
              (Semantics.Templates.NounPhrase.counted (Semantics.Templates.Quantity.exactly 2)
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isUntapped, Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl,
                    Semantics.Templates.Predicate.shares Semantics.QualitySort.color Semantics.Agreement.share
                      Semantics.Templates.NounPhrase.thisSpell]))))
          Bool.true Semantics.PayTimes.once),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisSpell))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.copyOnStack Semantics.Templates.NounPhrase.thisSpell,
            Semantics.Templates.Action.may (Semantics.Templates.Action.chooseNewTargets (Semantics.Templates.NounPhrase.that Semantics.NounWord.copy))])
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.conspire.label Semantics.Templates.NounPhrase.thisSpell))]
}
