/-- The object or player identified by the named designation. -/
primitive designated (designation : DesignationLabel) (whose : NounPhrase) : NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.designated designation whose
}
