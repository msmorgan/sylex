primitive partSkip (player : NounPhrase) (part : TurnPart) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.partSkip player part
}
