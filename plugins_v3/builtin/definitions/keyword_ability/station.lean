keyword station : Ability := {
  lexical := { lemma := "station", verbs := [{ lemma := "station", frames := [Monotransitive] }] },
  spelling := "station",
  body :=
    [Semantics.Templates.Ability.activatedOnlyDuring
        (Semantics.Templates.Cost.perform
          (Semantics.Definitions.tap
            (Semantics.Templates.NounPhrase.a
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isOther, Semantics.Templates.Predicate.isUntapped,
                  Semantics.Templates.Predicate.youControl]))))
        (Semantics.Templates.Action.putCounters
          (Semantics.Templates.Amount.powerOf
            (Semantics.Templates.NounPhrase.the
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.was (Semantics.Definitions.tap Semantics.Templates.NounPhrase.which),
                  Semantics.Templates.Predicate.isCreature])))
          (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.chargeCounter) Semantics.Templates.NounPhrase.thisPermanent)
        Semantics.Templates.Timing.asSorcery]
}
