/-- "<source> causes …": a spell, ability or other object as the cause. -/
primitive bySource (source : NounPhrase) : Causing := {
  -- TODO(spelling): Context-dependent wording: <source> causes …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Causing.source source
}
