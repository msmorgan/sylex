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
            (withTotal (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast
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
