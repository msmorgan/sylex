keyword mentor : Ability := {
  lexical := { lemma := "mentor", verbs := [{ lemma := "mentor", frames := [Monotransitive] }] },
  spelling := "mentor",
  body :=
    [whenever
        (does (actor thisCreature)
          (attack none))
        (putCounters 1 (printedKind p1p1Counter)
          (target
            (and
              [isCreature, isAttacking,
                hasPower .less
                  (powerOf thisCreature)])))]
}
