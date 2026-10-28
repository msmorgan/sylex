/-- Count distinct values of the specified axis in the domain. -/
primitive distinctCount (axis : KindAxis) (domain : NounPhrase) : Amount := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Amount.distinctCount axis domain
}
