/-- Authoring primitive for `StaticSpec.additionalTriggers`. -/
primitive additionalTriggers (event : GameEvent) (quantity : Quantity) : StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.additionalTriggers event quantity
}
