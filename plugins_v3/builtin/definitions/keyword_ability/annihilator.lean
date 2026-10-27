keyword annihilator (amount : Amount) : Ability := {
  lexical := { lemma := "annihilator", parameters := [[Amount]] },
  spelling := "annihilator <amount>",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack Option.none))
        (Semantics.Templates.Action.instruct Semantics.Templates.NounPhrase.defendingPlayer
          (Semantics.Definitions.sacrifice
            (Semantics.Templates.NounPhrase.counted (Semantics.Templates.Quantity.exactlyOf amount) Semantics.Templates.Predicate.isPermanent)))]
}
