keyword myriad : Ability := {
  lexical := { lemma := "myriad" },
  spelling := "myriad",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack Option.none))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.doForEach
              (Semantics.Templates.NounPhrase.each
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.isOpponent, Semantics.Templates.Predicate.otherThan Semantics.Templates.NounPhrase.defendingPlayer]))
              (Semantics.Templates.Action.may
                (Semantics.Definitions.create 1 (Semantics.Templates.TokenSpec.copyOf Semantics.Templates.NounPhrase.thisCreature)
                  [Semantics.Templates.MoveRider.entersAs Semantics.Status.tapped,
                    Semantics.Templates.MoveRider.entersAttacking
                      (Option.some
                        (Semantics.Templates.NounPhrase.or
                          [Semantics.Templates.NounPhrase.they,
                            Semantics.Templates.NounPhrase.a
                              (Semantics.Templates.Predicate.and
                                [Semantics.Templates.Predicate.isPlaneswalker,
                                  Semantics.Templates.Predicate.controlledBy Semantics.Templates.NounPhrase.they])]))])),
            Semantics.Templates.Action.doIf
              (Semantics.Templates.Condition.compareAmt (Semantics.Templates.Amount.numberOf Semantics.Templates.NounPhrase.thoseTokens)
                Semantics.Comparator.atLeast 1)
              (Semantics.Templates.Action.atNext Semantics.TurnPart.endOfCombat
                (Semantics.Definitions.exile Semantics.Templates.NounPhrase.thoseTokens))])]
}
