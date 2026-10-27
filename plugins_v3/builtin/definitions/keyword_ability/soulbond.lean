keyword soulbond : Ability := {
  lexical := { lemma := "soulbond" },
  spelling := "soulbond",
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature) Semantics.Templates.Action.enter)
        (Semantics.Templates.Action.may
          (Semantics.Templates.Action.relate Semantics.Templates.NounPhrase.thisCreature Semantics.Definitions.paired
            (Semantics.Templates.NounPhrase.a
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl, Semantics.Templates.Predicate.isOther],
                  Semantics.Templates.Predicate.not
                    (Semantics.Templates.Predicate.related Semantics.Definitions.paired Option.none)]))))
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.and
            [Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisCreature
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.youControl,
                    Semantics.Templates.Predicate.not
                      (Semantics.Templates.Predicate.related Semantics.Definitions.paired Option.none)]),
              Semantics.Templates.Condition.exists_
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.and
                      [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl, Semantics.Templates.Predicate.isOther],
                    Semantics.Templates.Predicate.not
                      (Semantics.Templates.Predicate.related Semantics.Definitions.paired Option.none)])])),
      Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does
          (Semantics.Templates.Subject.actor
            (Semantics.Templates.NounPhrase.a
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl, Semantics.Templates.Predicate.isOther])))
          Semantics.Templates.Action.enter)
        (Semantics.Templates.Action.may
          (Semantics.Templates.Action.relate (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature)
            Semantics.Definitions.paired Semantics.Templates.NounPhrase.thisCreature))
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.and
            [Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisCreature
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.youControl,
                    Semantics.Templates.Predicate.not
                      (Semantics.Templates.Predicate.related Semantics.Definitions.paired Option.none)]),
              Semantics.Templates.Condition.is (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature)
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.youControl,
                    Semantics.Templates.Predicate.not
                      (Semantics.Templates.Predicate.related Semantics.Definitions.paired Option.none)])]))]
}
