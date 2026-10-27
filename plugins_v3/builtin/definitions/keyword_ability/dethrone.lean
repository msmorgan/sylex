keyword dethrone : Ability := {
  lexical := { lemma := "dethrone" },
  spelling := "dethrone",
  body :=
    [ whenever
        (does thisCreature
          (.attack
            (some
              (a (and [.isPlayer, .superlative .max (.playerStat .lifeTotal) .isPlayer])))))
        (putCounters 1 p1p1Counter thisCreature) ]
}
