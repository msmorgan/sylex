primitive letterDefinition (letter : Letter) (amount : Amount) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.letterDefinition letter amount
}
