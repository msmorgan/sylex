keyword graft (amount : Amount) : Ability := {
  lexical := { lemma := "graft", parameters := [[Amount]] },
  spelling := "graft <amount>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.entersWithCounters Semantics.Templates.NounPhrase.thisPermanent amount Semantics.Definitions.p1p1Counter),
      Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does
          (Semantics.Templates.Subject.actor
            (Semantics.Templates.NounPhrase.a
              (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isOther])))
          Semantics.Templates.Action.enter)
        (Semantics.Templates.Action.may
          (Semantics.Templates.Action.moveCounters 1 Semantics.Templates.NounPhrase.thisPermanent
            (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature)
            (Option.some (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter))))
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent
            (Semantics.Templates.Predicate.hasCounter (Option.some Semantics.Definitions.p1p1Counter))))]
}
