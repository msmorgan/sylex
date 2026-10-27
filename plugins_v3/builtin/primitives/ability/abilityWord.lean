/-- An ability word in italics before an ability: "Will of the council — …" [CR#207.2c]. -/
-- [CR#207.2c]
primitive abilityWord (word : AbilityWordLabel) (ability : Ability) : Ability := {
  -- TODO(spelling): Context-dependent wording: Will of the council — …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Ability.italicHead (Semantics.ItalicWord.abilityWord word) ability
}
