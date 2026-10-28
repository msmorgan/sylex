/-- Authoring primitive for `StaticSpec.manaRetention`. -/
primitive manaRetention (player : NounPhrase) (mana : ManaHeld) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.manaRetention player mana
}
