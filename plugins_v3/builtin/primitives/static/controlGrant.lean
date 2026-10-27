primitive controlGrant (player subject : NounPhrase) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.controlGrant player subject
}
