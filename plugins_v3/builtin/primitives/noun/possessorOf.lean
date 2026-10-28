/-- The owner or controller of the subject, as selected by the axis. -/
primitive possessorOf (axis : PossessorAxis) (subject : NounPhrase) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.possessorOf axis subject
}
