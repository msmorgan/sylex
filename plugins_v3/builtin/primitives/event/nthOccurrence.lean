/-- The specified occurrence of an event within an optional period. -/
primitive nthOccurrence (ordinal : Ordinal) (event : GameEvent)
    (per : Option Period := none) : GameEvent := {
  body :=
    Semantics.Primitives.GameEvent.nthOccurrence ordinal per event
}
