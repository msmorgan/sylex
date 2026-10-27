keyword extort : Ability := {
  lexical := { lemma := "extort" },
  spelling := "extort",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isSpell)))
        (Semantics.Templates.Action.may
          (Semantics.Templates.Action.pay
            (Semantics.Templates.Cost.mana [Semantics.Templates.ManaSymbol.hybridPip Semantics.Color.white Semantics.Color.black])
            Semantics.PayTimes.once)
          (Option.some
            (Semantics.Templates.Action.sequentially
              [Semantics.Templates.Action.instruct Semantics.Templates.NounPhrase.eachOpponent (Semantics.Templates.Action.loseLife 1),
                Semantics.Templates.Action.gainLife Semantics.Templates.Amount.thatMuch])))]
}
