/-- "an <parity> mana value" [CR#202.3] -/
-- [CR#202.3]
primitive manaValueParity (parity : Parity) : DeckTrait := {
  spelling := "an <parity> mana value",
  body :=
    Semantics.Primitives.DeckTrait.manaValueParity parity
}
