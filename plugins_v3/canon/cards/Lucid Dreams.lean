/-- Lucid Dreams -/
card := {
  name := "Lucid Dreams",
  cost := [3, blue, blue],
  types := [sorcery],
  text := [spell [draw (letter x), define x (distinctCount cardType (allOf (cardIn yourGraveyard)))]]
}
