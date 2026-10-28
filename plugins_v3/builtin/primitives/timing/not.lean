/-- The timing constraint does not apply. -/
primitive not (timing : Timing) : Timing := {
  body :=
    Semantics.Primitives.Timing.not timing
}
