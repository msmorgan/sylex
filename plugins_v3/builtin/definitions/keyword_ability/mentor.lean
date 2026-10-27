keyword mentor : Ability := {
  lexical := { lemma := "mentor", verbs := [{ lemma := "mentor", frames := [Monotransitive] }] },
  spelling := "mentor",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack Option.none))
        (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          (Semantics.Templates.NounPhrase.target
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isAttacking,
                Semantics.Templates.Predicate.hasPower Semantics.Comparator.less
                  (Semantics.Templates.Amount.powerOf Semantics.Templates.NounPhrase.thisCreature)])))]
}
