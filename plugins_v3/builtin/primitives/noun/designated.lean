/-- The object or player identified by the named designation. -/
primitive designated (designation : DesignationLabel) (whose : NounPhrase) : NounPhrase := {
  body :=
    Semantics.Primitives.NounPhrase.designated designation whose
}
