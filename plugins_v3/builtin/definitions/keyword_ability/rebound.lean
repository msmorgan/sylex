keyword rebound : Ability := {
  lexical := { lemma := "rebound" },
  spelling := "rebound",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.causedBy
            (Semantics.Templates.Causing.byEvent
              (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
                (Semantics.Definitions.resolve Semantics.Templates.NounPhrase.thisSpell)))
            (Semantics.Templates.WouldClause.would Semantics.Templates.Subject.any
              (Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.thisSpell Semantics.Templates.Location.graveyard
                (Option.some Semantics.Templates.Location.stack))))
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisSpell
              (Semantics.Templates.Predicate.did Semantics.Templates.Subject.any
                (Semantics.Definitions.cast Semantics.Templates.NounPhrase.which (Option.some Semantics.Templates.Location.yourHand))
                (Option.some Semantics.Templates.LookbackSpan.thisObject)))
            (Semantics.Templates.Replacement.instead
              (Semantics.Templates.Action.sequentially
                [Semantics.Definitions.exile Semantics.Templates.NounPhrase.thisSpell,
                  Semantics.Templates.Action.delay
                    (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.upkeep (Option.some Semantics.Templates.NounPhrase.you)
                      (Semantics.Templates.TurnRef.nextTime Option.none))
                    (Semantics.Templates.Action.may
                      (Semantics.Definitions.cast Semantics.Templates.NounPhrase.this_ Option.none
                        Semantics.Templates.PlayPayment.withoutPaying))]))))]
}
