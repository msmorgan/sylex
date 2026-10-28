keyword dethrone : Ability := {
  lexical := { lemma := "dethrone" },
  spelling := "dethrone",
  body :=
    [whenever
        (does (actor thisCreature)
          (attack
            (some
              (a
                (and
                  [isPlayer,
                    superlative Semantics.AggregateOp.max
                      (Semantics.ProjAxis.playerStat Semantics.PlayerStat.lifeTotal) isPlayer])))))
        (putCounters 1 (printedKind p1p1Counter)
          thisCreature)]
}
