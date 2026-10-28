card := {
  name := "Ensnaring Bridge",
  cost := [3],
  types := [artifact],
  text := [cant (allOf [isCreature, hasPower greater (countOf (cardIn yourHand))]) attack]
}
