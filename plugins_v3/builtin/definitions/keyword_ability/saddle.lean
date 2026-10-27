keyword saddle (amount : Amount) : Ability := {
  lexical := {
    lemma := "saddle",
    parameters := [[Amount]],
    verbs := [{ lemma := "saddle", frames := [Monotransitive] }]
  },
  spelling := "saddle <amount>",
  body :=
    [Semantics.Templates.Ability.activatedOnlyDuring
        (Semantics.Templates.Cost.perform
          (Semantics.Definitions.tap
            (Semantics.Templates.NounPhrase.withTotal (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast
              amount
              (Semantics.Templates.NounPhrase.allOf
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isUntapped,
                    Semantics.Templates.Predicate.and
                      [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl, Semantics.Templates.Predicate.isOther]])))))
        (Semantics.Templates.Action.gainDesignation Semantics.Templates.NounPhrase.thisPermanent Semantics.Definitions.saddled
          (Option.some Semantics.Templates.Duration.untilEndOfTurn))
        Semantics.Templates.Timing.asSorcery]
}
