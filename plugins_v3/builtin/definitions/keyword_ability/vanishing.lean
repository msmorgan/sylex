keyword vanishing (amount : Amount) : Ability := {
  lexical := { lemma := "vanishing", parameters := [[], [Amount]] },
  spelling := "vanishing <amount>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.entersWithCounters Semantics.Templates.NounPhrase.thisPermanent amount Semantics.Definitions.timeCounter),
      Semantics.Templates.Ability.at_ (Semantics.Templates.GameEvent.beginningOfYour Semantics.TurnPart.upkeep)
        (Semantics.Templates.Action.removeCounters (Semantics.Templates.Quantity.oneThrough 1)
          (Option.some (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.timeCounter))
          Semantics.Templates.NounPhrase.thisPermanent)
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent
            (Semantics.Templates.Predicate.hasCounter (Option.some Semantics.Definitions.timeCounter)))),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
          (Semantics.Templates.Action.removeCounters Semantics.Templates.Quantity.theLast
            (Option.some (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.timeCounter))
            Semantics.Templates.NounPhrase.thisPermanent))
        (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thisPermanent)]
}
