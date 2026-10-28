/-- "Mentor" means "Whenever this creature attacks, put a +1/+1 counter on target attacking creature
with power less than this creature's power." [CR#702.134a] -/
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
