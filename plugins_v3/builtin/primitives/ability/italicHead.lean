/-- Attach an italic heading to an ability. -/
primitive italicHead (word : ItalicWord) (ability : Ability) : Ability := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Ability.italicHead word ability
}
