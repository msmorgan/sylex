keyword crew (amount : Amount) : Ability := {
  lexical := {
    lemma := "crew",
    parameters := [[Amount]],
    verbs := [{ lemma := "crew", frames := [Monotransitive] }]
  },
  spelling := "crew <amount>",
  body :=
    [activated
        (perform
          (tap
            (withTotal (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast
              amount
              (allOf
                (and
                  [isUntapped,
                    and
                      [isCreature, youControl, isOther]])))))
        (establishThroughout
          (characteristicChange thisPermanent
            [characteristicEditTypeLine Semantics.QualityOp.adds
                { types := some [Semantics.CardType.artifact, Semantics.CardType.creature] }])
          untilEndOfTurn)]
}
