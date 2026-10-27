keyword crew (amount : Amount) : Ability := {
  lexical := {
    lemma := "crew",
    parameters := [[Amount]],
    verbs := [{ lemma := "crew", frames := [Monotransitive] }]
  },
  spelling := "crew <amount>",
  body :=
    [ activated
        (.perform
          (tap
            (withTotal (.stat .power) .atLeast amount
              (allOf
                (and
                  [ isUntapped,
                    and [isCreature, youControl, isOther] ])))))
        (establishThroughout
          (.characteristicChange thisPermanent
            [.typeLine .adds { types := some [.artifact, .creature] }])
          untilEndOfTurn) ]
}
