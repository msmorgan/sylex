/-- "if no colored mana was spent to cast <spell>" -/
template noColoredManaSpentToCast (spell : NounPhrase) : Condition := {
  spelling := "if no colored mana was spent to cast <spell>",
  body :=
    compareAmt (colorsSpentToCast spell) .eq 0
}
