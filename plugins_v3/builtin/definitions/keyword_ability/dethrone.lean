keyword dethrone : Ability := {
  lexical := { lemma := "dethrone" },
  spelling := "dethrone",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack
            (Option.some
              (Semantics.Templates.NounPhrase.a
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isPlayer,
                    Semantics.Templates.Predicate.superlative Semantics.AggregateOp.max
                      (Semantics.ProjAxis.playerStat Semantics.PlayerStat.lifeTotal) Semantics.Templates.Predicate.isPlayer])))))
        (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          Semantics.Templates.NounPhrase.thisCreature)]
}
