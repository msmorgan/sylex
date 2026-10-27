keyword impending (amount : Amount) (cost : Cost) : Ability := {
  lexical := { lemma := "impending", parameters := [[Amount, DashCost]] },
  spelling := "impending <amount>—<cost>",
  body :=
    [Semantics.Templates.Ability.static (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_ (Option.some cost)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            Semantics.Templates.Action.enter)
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.impending.label Semantics.Templates.NounPhrase.thisPermanent)
            (Semantics.Templates.Replacement.withRiders
              [Semantics.Templates.MoveRider.withCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.timeCounter)
                  Semantics.EntryCounterMark.fresh]))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.characteristicChange Semantics.Templates.NounPhrase.thisPermanent
            [Semantics.Templates.CharacteristicEdit.characteristicEditTypeLine Semantics.QualityOp.loses
                { types := Option.some [Semantics.CardType.creature] }])
          (Semantics.Templates.Condition.and
            [Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.impending.label Semantics.Templates.NounPhrase.thisPermanent,
              Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent
                (Semantics.Templates.Predicate.hasCounter (Option.some Semantics.Definitions.timeCounter))])),
      Semantics.Templates.Ability.at_ (Semantics.Templates.GameEvent.beginningOfYour Semantics.TurnPart.endStep)
        (Semantics.Templates.Action.removeCounters (Semantics.Templates.Quantity.oneThrough 1)
          (Option.some (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.timeCounter))
          Semantics.Templates.NounPhrase.thisPermanent)
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.and
            [Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.impending.label Semantics.Templates.NounPhrase.thisPermanent,
              Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent
                (Semantics.Templates.Predicate.hasCounter (Option.some Semantics.Definitions.timeCounter))]))]
}
