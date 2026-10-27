primitive withSubstitute (cost : Cost) (substitute : ManaSubstitute) : Cost := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Cost.withSubstitute cost substitute
}
