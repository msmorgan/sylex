/-- Authoring primitive for `StaticSpec.manaRetention`. -/
primitive manaRetention (player : NounPhrase) (mana : ManaHeld) : StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.manaRetention player mana
}
