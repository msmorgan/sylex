keyword station : Ability := {
  lexical := { lemma := "station", verbs := [{ lemma := "station", frames := [Monotransitive] }] },
  spelling := "station",
  body :=
    [ activatedOnlyDuring
        (.perform (tap (a (and [isCreature, isOther, isUntapped, youControl]))))
        (putCounters (powerOf (the (and [was (tap which), isCreature])))
          chargeCounter thisPermanent)
        asSorcery ]
}
