/-- Pay the cost, using the substitute only for that cost's own generic mana, as with waterbend
[CR#701.67b]. -/
primitive withSubstitute (cost : Cost) (substitute : ManaSubstitute) : Cost := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Cost.withSubstitute cost substitute
}
