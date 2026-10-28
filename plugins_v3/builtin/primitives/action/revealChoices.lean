/-- Reveal the choices of the specified hidden sort. -/
primitive revealChoices (sort : HiddenSort) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.revealChoices sort
}
