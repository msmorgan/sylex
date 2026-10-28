/-- A prototype frame's inset set: its mana cost and power/toughness box [CR#718.1]. -/
-- [CR#718.1]
template prototypeAlt (cost : ManaCost) (power toughness : Nat) : PrototypeFrame := {
  -- TODO(spelling): A prototype frame's inset set: its mana cost and power/toughness box .
  body :=
    { cost := some cost, power := stat power, toughness := stat toughness }
}
