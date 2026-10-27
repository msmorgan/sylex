/-- The replaced entering still happens, with Entry Riders `riders`: "enters tapped", "enters
with N counters", "enters as a copy", "enters transformed" [CR#614.1c,614.1d]. -/
-- [CR#614.1c,614.1d]
primitive withRiders (riders : List MoveRider) : Replacement := {
  -- TODO(spelling): enters tapped; wording varies with riders, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Replacement.withRiders riders
}
