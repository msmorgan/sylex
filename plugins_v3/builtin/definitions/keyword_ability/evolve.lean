keyword evolve : Ability := {
  lexical := { lemma := "evolve", verbs := [{ lemma := "evolve", frames := [Intransitive] }] },
  spelling := "evolve",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does
          (Semantics.Templates.Subject.actor
            (Semantics.Templates.NounPhrase.a
              (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl])))
          Semantics.Templates.Action.enter)
        (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          Semantics.Templates.NounPhrase.thisCreature)
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.or
            [Semantics.Templates.Condition.compareAmt
                (Semantics.Templates.Amount.powerOf (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature))
                Semantics.Comparator.greater (Semantics.Templates.Amount.powerOf Semantics.Templates.NounPhrase.thisCreature),
              Semantics.Templates.Condition.compareAmt
                (Semantics.Templates.Amount.toughnessOf (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature))
                Semantics.Comparator.greater (Semantics.Templates.Amount.toughnessOf Semantics.Templates.NounPhrase.thisCreature)]))]
}
