keyword training : Ability := {
  lexical := { lemma := "training", verbs := [{ lemma := "train", frames := [Intransitive] }] },
  spelling := "training",
  body :=
    [whenever
        (does
          (actor
            (and
              [thisCreature,
                counted (orMore 1)
                  (and
                    [isCreature, isOther,
                      hasPower .greater
                        (powerOf thisCreature)])]))
          (attack none))
        (putCounters 1 (printedKind p1p1Counter)
          thisCreature)]
}
