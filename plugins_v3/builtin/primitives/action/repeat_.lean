/-- Repeat according to the specified repetition. -/
primitive repeat_ (repetition : Repetition) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.repeat_ repetition
}
