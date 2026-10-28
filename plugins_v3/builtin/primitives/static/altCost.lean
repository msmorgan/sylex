/-- Authoring primitive for `StaticSpec.altCost`. -/
primitive altCost (subject : NounPhrase) (cost : Option Cost := none) : StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.altCost subject cost
}
