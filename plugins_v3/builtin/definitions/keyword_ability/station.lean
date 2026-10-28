/-- "Station" means "Tap another untapped creature you control: Put a number of charge counters on
this permanent equal to the tapped creature's power. Activate only as a sorcery." [CR#702.184a] -/
keyword station : Ability := {
  lexical := { lemma := "station", verbs := [{ lemma := "station", frames := [Monotransitive] }] },
  spelling := "station",
  body :=
    [activatedOnlyDuring
        (perform
          (tap
            (a
              (and
                [isCreature, isOther, isUntapped,
                  youControl]))))
        (putCounters
          (powerOf
            (the
              (and
                [was (tap which),
                  isCreature])))
          (printedKind chargeCounter) thisPermanent)
        asSorcery]
}
