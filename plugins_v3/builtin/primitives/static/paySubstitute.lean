primitive paySubstitute (subject : NounPhrase) (substitute : ManaSubstitute) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.paySubstitute subject substitute
}
