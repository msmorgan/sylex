/-- "for each color of mana spent to cast <n>" -/
template colorsSpentToCast (spell : NounPhrase) : Amount := {
  spelling := "the number of colors of mana spent to cast <spell>",
  body :=
    paid colorsSpent spell
}
