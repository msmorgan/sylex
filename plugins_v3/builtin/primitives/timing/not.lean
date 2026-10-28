/-- The timing constraint does not apply. -/
primitive not (timing : Timing) : Timing := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Timing.not timing
}
