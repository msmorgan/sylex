primitive nthOccurrence (ordinal : Ordinal) (event : GameEvent)
    (per : Option Period := none) : GameEvent := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.GameEvent.nthOccurrence ordinal per event
}
