/-- Authoring primitive for `StaticSpec.additionalTriggers`. -/
primitive additionalTriggers (event : GameEvent) (quantity : Quantity) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.additionalTriggers event quantity
}
