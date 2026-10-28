/-- The object that granted this ability, identified by its head noun. -/
primitive theGrantor (head : NounWord) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.theGrantor head
}
