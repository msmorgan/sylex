keyword poisonous (amount : Amount) : Ability := {
  lexical := { lemma := "poisonous", parameters := [[Amount]] },
  spelling := "poisonous <amount>",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer)
            (Option.some Semantics.DamageKind.combat)))
        (Semantics.Templates.Action.putCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.poison)
          Semantics.Templates.NounPhrase.they)]
}
