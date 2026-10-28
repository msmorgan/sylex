/-- "Saddle N" means "Tap any number of other untapped creatures you control with total power N or
greater: This permanent becomes saddled until end of turn. Activate only as a sorcery."
[CR#702.171a] -/
keyword saddle (amount : Amount) : Ability := {
  lexical := {
    lemma := "saddle",
    parameters := [[Amount]],
    verbs := [{ lemma := "saddle", frames := [Monotransitive] }]
  },
  spelling := "saddle <amount>",
  body :=
    [activatedOnlyDuring
        (perform
          (tap
            (withTotal (.stat power) atLeast
              amount
              (allOf
                (and
                  [isUntapped,
                    and
                      [isCreature, youControl, isOther]])))))
        (gainDesignation thisPermanent saddled
          (some untilEndOfTurn))
        asSorcery]
}
