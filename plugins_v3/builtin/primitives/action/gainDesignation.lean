/-- Give the subject the designation for the given duration. -/
primitive gainDesignation (subject : NounPhrase) (designation : DesignationLabel)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.gainDesignation subject designation duration
}
