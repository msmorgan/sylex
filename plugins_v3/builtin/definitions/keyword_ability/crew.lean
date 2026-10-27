keyword crew (amount : Amount) : Ability := {
  lexical := {
    lemma := "crew",
    parameters := [[Amount]],
    verbs := [{ lemma := "crew", frames := [Monotransitive] }]
  },
  spelling := "crew <amount>",
  body :=
    [Semantics.Templates.Ability.activated
        (Semantics.Templates.Cost.perform
          (Semantics.Definitions.tap
            (Semantics.Templates.NounPhrase.withTotal (Semantics.ProjAxis.stat Semantics.Stat.power) Semantics.Comparator.atLeast
              amount
              (Semantics.Templates.NounPhrase.allOf
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isUntapped,
                    Semantics.Templates.Predicate.and
                      [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl, Semantics.Templates.Predicate.isOther]])))))
        (Semantics.Templates.Action.establishThroughout
          (Semantics.Templates.StaticSpec.characteristicChange Semantics.Templates.NounPhrase.thisPermanent
            [Semantics.Templates.CharacteristicEdit.characteristicEditTypeLine Semantics.QualityOp.adds
                { types := Option.some [Semantics.CardType.artifact, Semantics.CardType.creature] }])
          Semantics.Templates.Duration.untilEndOfTurn)]
}
