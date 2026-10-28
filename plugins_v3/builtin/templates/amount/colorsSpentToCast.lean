/-- "for each color of mana spent to cast <n>" -/
template colorsSpentToCast (spell : NounPhrase) : Amount := {
  -- TODO(spelling): Context-dependent wording: for each color of mana spent to cast <n>; string substitution cannot express its omitted or inflected parts.
  body :=
    paid colorsSpent spell
}
