/-- "if colored mana was spent to cast <spell>" -/
template coloredManaSpentToCast (spell : NounPhrase) : Condition := {
  spelling := "if colored mana was spent to cast <spell>",
  body :=
    compareAmt (colorsSpentToCast spell) Semantics.Comparator.atLeast 1
}
