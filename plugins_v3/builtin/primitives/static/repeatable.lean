/-- Allows the same mode to be chosen more than once. [CR#700.2d] -/
primitive repeatable : ModeChange := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.ModeChange.repeatable
}
