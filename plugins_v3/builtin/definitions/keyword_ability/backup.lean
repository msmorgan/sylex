keyword backup (amount : Amount) : Ability := {
  lexical := { lemma := "backup", parameters := [[Amount]] },
  spelling := "backup <amount>",
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature) Semantics.Templates.Action.enter)
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.putCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
              (Semantics.Templates.NounPhrase.target Semantics.Templates.Predicate.isCreature),
            Semantics.Templates.Action.doIf
              (Semantics.Templates.Condition.is (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature)
                (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isOther]))
              (Semantics.Templates.Action.establishThroughout
                (Semantics.Templates.StaticSpec.abilityGrantSelected (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature)
                  Semantics.Templates.NounPhrase.thisCreature
                  (Semantics.Templates.AbilitySelection.abilitySelectionPrintedAfter Semantics.Definitions.backup.label))
                Semantics.Templates.Duration.untilEndOfTurn)])]
}
