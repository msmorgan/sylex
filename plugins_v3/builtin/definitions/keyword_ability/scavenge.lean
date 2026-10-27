keyword scavenge (cost : Cost) : Ability := {
  lexical := { lemma := "scavenge", parameters := [[Cost]] },
  spelling := "scavenge <cost>",
  body :=
    [Semantics.Templates.Ability.activatedOnlyDuring
        (Semantics.Templates.Cost.compound
          [cost,
            Semantics.Templates.Cost.perform
              (Semantics.Definitions.exileFrom Semantics.Templates.NounPhrase.thisCard Semantics.Templates.Location.yourGraveyard)])
        (Semantics.Templates.Action.putCounters
          (Semantics.Templates.Amount.powerOf
            (Semantics.Templates.NounPhrase.the
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCard,
                  Semantics.Templates.Predicate.did (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
                    (Semantics.Definitions.exile Semantics.Templates.NounPhrase.which)])))
          (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          (Semantics.Templates.NounPhrase.target Semantics.Templates.Predicate.isCreature))
        Semantics.Templates.Timing.asSorcery]
}
