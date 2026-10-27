keyword casualty (amount : Amount) : Ability := {
  lexical := { lemma := "casualty", parameters := [[Amount]] },
  spelling := "casualty <amount>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.addedCost
          (Semantics.Templates.Cost.perform
            (Semantics.Definitions.sacrifice
              (Semantics.Templates.NounPhrase.a
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.hasPower Semantics.Comparator.atLeast amount]))))
          Bool.true Semantics.PayTimes.once),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisSpell))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.copyOnStack Semantics.Templates.NounPhrase.thisSpell,
            Semantics.Templates.Action.may (Semantics.Templates.Action.chooseNewTargets (Semantics.Templates.NounPhrase.that Semantics.NounWord.copy))])
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.casualty.label Semantics.Templates.NounPhrase.thisSpell))]
}
