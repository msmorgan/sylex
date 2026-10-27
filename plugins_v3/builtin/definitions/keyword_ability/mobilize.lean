keyword mobilize (amount : Amount) : Ability := {
  lexical := { lemma := "mobilize", parameters := [[Amount]] },
  spelling := "mobilize <amount>",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack Option.none))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.createTappedAttacking amount
              (Semantics.Templates.CharacteristicBundle.creatureToken 1 1 [Semantics.Color.red] [Semantics.Definitions.warrior]),
            Semantics.Templates.Action.atNext Semantics.TurnPart.endStep
              (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thoseTokens)])]
}
