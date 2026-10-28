/-- "this turn", "this game": the span a historical clause looks back over. -/
primitive within (d : Duration) (timing : Option Timing := none) : LookbackSpan := {
  -- TODO(spelling): Include or omit the clauses supplied by timing.
  body :=
    Semantics.Primitives.LookbackSpan.duration d timing
}
