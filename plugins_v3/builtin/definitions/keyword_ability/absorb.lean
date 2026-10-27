/- ## Keyword abilities -/

/-- "Absorb N": "If a source would deal damage to this creature, prevent N of that damage."
[CR#702.64a] -/
keyword absorb (amount : Amount) : Ability := {
  lexical := { lemma := "absorb", parameters := [[Amount]] },
  spelling := "absorb <amount>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isSource))
            (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount Semantics.Templates.NounPhrase.thisCreature))
          (Semantics.Templates.Replacement.instead (Semantics.Templates.Action.preventDamage (Semantics.Templates.PreventCut.ofThatDamage amount))))]
}
