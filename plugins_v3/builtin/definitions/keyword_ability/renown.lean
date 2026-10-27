/-- "Renown N" with its reminder text: "When this creature deals combat damage to a player, if it
isn't renowned, put N +1/+1 counters on it and it becomes renowned." [CR#702.112a] -/
keyword renown (count : Amount) : Ability := {
  lexical := { lemma := "renown", parameters := [[Amount]] },
  spelling := "renown <count>",
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer)
            (Option.some Semantics.DamageKind.combat)))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.putCounters count (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
              Semantics.Templates.NounPhrase.thisCreature,
            Semantics.Templates.Action.gainDesignation Semantics.Templates.NounPhrase.thisCreature Semantics.Definitions.renowned])
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.not
            (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisCreature
              (Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.renowned))))]
}
