keyword evolve : Ability := {
  lexical := { lemma := "evolve", verbs := [{ lemma := "evolve", frames := [Intransitive] }] },
  spelling := "evolve",
  body :=
    [whenever
        (does
          (actor
            (a
              (and [isCreature, youControl])))
          enter)
        (putCounters 1 (printedKind p1p1Counter)
          thisCreature)
        [] [] none none none
        (some
          (or
            [compareAmt
                (powerOf (that creature))
                .greater (powerOf thisCreature),
              compareAmt
                (toughnessOf (that creature))
                .greater (toughnessOf thisCreature)]))]
}
