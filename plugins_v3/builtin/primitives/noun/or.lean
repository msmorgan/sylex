primitive or (phrases : List NounPhrase := []) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.coord Semantics.Coordinator.or phrases
}
