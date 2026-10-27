action bolster (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "bolster", frames := [Amount] }] },
  spelling := "bolster <amount>",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.choose
          (Semantics.Templates.NounPhrase.a
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.actorControls,
                Semantics.Templates.Predicate.superlative Semantics.AggregateOp.min (Semantics.ProjAxis.stat Semantics.Stat.toughness)
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.actorControls])])),
        Semantics.Templates.Action.putCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature)]
}
