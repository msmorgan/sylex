/-- One level band of a leveler: its range, power/toughness box, and text [CR#711.2a]. -/
-- [CR#711.2a]
template levelBand (range : LevelRange) (power toughness : Nat) (text : List Ability) :
    LevelBand := {
  -- TODO(spelling): One level band of a leveler: its range, power/toughness box, and text .
  body :=
    { range := range, text := text, power := stat power,
      toughness := stat toughness }
}
