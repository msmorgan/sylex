keyword unleash : Ability := {
  lexical := { lemma := "unleash" },
  spelling := "unleash",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            Semantics.Templates.Action.enter)
          (Semantics.Templates.Replacement.alongside
            (Semantics.Templates.Action.may
              (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
                Semantics.Templates.NounPhrase.thisPermanent)))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.cant (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            (Semantics.Templates.Action.block Semantics.Templates.NounPhrase.any))
          (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent
            (Semantics.Templates.Predicate.hasCounter (Option.some Semantics.Definitions.p1p1Counter))))]
}
