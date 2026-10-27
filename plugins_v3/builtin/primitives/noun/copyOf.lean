primitive copyOf (source : NounPhrase) (exceptions : List CopyExcept := []) : TokenSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.TokenSpec.copyOf source exceptions
}
