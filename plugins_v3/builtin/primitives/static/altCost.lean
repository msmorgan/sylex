primitive altCost (subject : NounPhrase) (cost : Option Cost := none) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.altCost subject cost
}
