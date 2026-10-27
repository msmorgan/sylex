keyword evolve : Ability := {
  lexical := { lemma := "evolve", verbs := [{ lemma := "evolve", frames := [Intransitive] }] },
  spelling := "evolve",
  body :=
    [ whenever (does (a (and [isCreature, youControl])) enter)
        (if_ := some (or
          [ .compareAmt (powerOf (that creature)) .greater (powerOf thisCreature),
            .compareAmt (toughnessOf (that creature)) .greater (toughnessOf thisCreature) ]))
        (putCounters 1 p1p1Counter thisCreature) ]
}
