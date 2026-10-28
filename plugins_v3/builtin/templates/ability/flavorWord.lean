/-- A flavor word in italics before an ability [CR#207.2d]. -/
-- [CR#207.2d]
template flavorWord (word : FlavorWordLabel) (ability : Ability) : Ability := {
  -- TODO(spelling): A flavor word in italics before an ability .
  body := italicHead (.flavorWord word) ability
}
