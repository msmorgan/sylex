/-- Attach an italic heading to an ability. -/
primitive italicHead (word : ItalicWord) (ability : Ability) : Ability := {
  body :=
    Semantics.Primitives.Ability.italicHead word ability
}
