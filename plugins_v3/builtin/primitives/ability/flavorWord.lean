/-- A flavor word in italics before an ability [CR#207.2d]. -/
-- [CR#207.2d]
primitive flavorWord (word : FlavorWordLabel) (ability : Ability) : Ability := {
  -- TODO(spelling): A flavor word in italics before an ability .
  body :=
    Semantics.Primitives.Ability.italicHead (Semantics.ItalicWord.flavorWord word) ability
}
