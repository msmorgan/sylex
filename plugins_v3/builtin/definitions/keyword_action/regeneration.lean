/-- "the regeneration of <subject>": a regeneration shield's application, "remove all damage
marked on it and tap it. If it's an attacking or blocking creature, remove it from combat"
[CR#701.19a], the deed "can't be regenerated" forbids. Activating or casting what creates a
shield is not regenerating (`regenerate`, [CR#701.19c]) (`Definition`). -/
action regeneration (subject : NounPhrase) : Action := {
  -- TODO(spelling): the regeneration of <subject>; this is shield application, not the Oracle
  -- instruction that creates a regeneration shield.
  lexical := { verbs := [{ lemma := "regenerate", frames := [Intransitive, Monotransitive] }] },
  deed := "regenerate",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.clearDamage subject,
        Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.possessorOf Semantics.PossessorAxis.controller subject)
          (Semantics.Definitions.tap subject),
        Semantics.Templates.Action.doIf
          (Semantics.Templates.Condition.or
            [Semantics.Templates.Condition.is subject (Semantics.Templates.Predicate.inCombat Semantics.CombatRelation.attackerOf Option.none),
              Semantics.Templates.Condition.is subject (Semantics.Templates.Predicate.inCombat Semantics.CombatRelation.blockerOf Option.none)])
          (Semantics.Templates.Action.removeFromCombat subject)]
}
