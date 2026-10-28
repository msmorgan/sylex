/-- "<source> causes …": a spell, ability or other object as the cause. -/
primitive bySource (source : NounPhrase) : Causing := {
  body :=
    Semantics.Primitives.Causing.source source
}
