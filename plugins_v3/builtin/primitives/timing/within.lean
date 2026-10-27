/-- "this turn", "this game": the span a historical clause looks back over. -/
primitive within (d : Duration) (timing : Option Timing := none) : LookbackSpan := {
  -- TODO(spelling): this turn; wording varies with d, timing, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.LookbackSpan.duration d timing
}
