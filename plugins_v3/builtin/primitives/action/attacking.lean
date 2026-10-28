primitive attacking (defender : Option NounPhrase) : CombatParticipation := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CombatParticipation.attacking defender
}
