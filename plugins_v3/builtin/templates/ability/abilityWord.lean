/-- An ability word in italics before an ability: "Will of the council — …" [CR#207.2c]. -/
-- [CR#207.2c]
template abilityWord (word : AbilityWordLabel) (ability : Ability) : Ability := {
  spelling := "<word> — <ability>",
  body := italicHead (.abilityWord word) ability
}
