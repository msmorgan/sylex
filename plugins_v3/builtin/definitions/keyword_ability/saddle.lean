keyword saddle (amount : Amount) : Ability := {
  lexical := {
    lemma := "saddle",
    parameters := [[Amount]],
    verbs := [{ lemma := "saddle", frames := [Monotransitive] }]
  },
  spelling := "saddle <amount>",
  body :=
    [ activatedOnlyDuring
        (.perform
          (tap
            (withTotal (.stat .power) .atLeast amount
              (allOf
                (and
                  [ isUntapped,
                    and [isCreature, youControl, isOther] ])))))
        (gainDesignation thisPermanent saddled untilEndOfTurn)
        asSorcery ]
}
