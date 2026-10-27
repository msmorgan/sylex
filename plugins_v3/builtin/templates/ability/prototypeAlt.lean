/-- A prototype frame's inset set: its mana cost and power/toughness box [CR#718.1]. -/
-- [CR#718.1]
template prototypeAlt (cost : ManaCost) (power toughness : Nat) : PrototypeFrame := {
  -- TODO(spelling): A prototype frame's inset set: its mana cost and power/toughness box .
  body :=
    { cost := Option.some cost, power := Semantics.Templates.stat power, toughness := Semantics.Templates.stat toughness }
}
