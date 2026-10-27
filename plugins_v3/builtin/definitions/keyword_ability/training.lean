keyword training : Ability := {
  lexical := { lemma := "training", verbs := [{ lemma := "train", frames := [Intransitive] }] },
  spelling := "training",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does
          (Semantics.Templates.Subject.actor
            (Semantics.Templates.NounPhrase.and
              [Semantics.Templates.NounPhrase.thisCreature,
                Semantics.Templates.NounPhrase.counted (Semantics.Templates.Quantity.orMore 1)
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isOther,
                      Semantics.Templates.Predicate.hasPower Semantics.Comparator.greater
                        (Semantics.Templates.Amount.powerOf Semantics.Templates.NounPhrase.thisCreature)])]))
          (Semantics.Templates.Action.attack Option.none))
        (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          Semantics.Templates.NounPhrase.thisCreature)]
}
