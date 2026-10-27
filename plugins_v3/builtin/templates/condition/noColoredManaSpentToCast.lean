/-- "if no colored mana was spent to cast <spell>" -/
template noColoredManaSpentToCast (spell : NounPhrase) : Condition := {
  spelling := "if no colored mana was spent to cast <spell>",
  body :=
    Semantics.Templates.Condition.compareAmt (Semantics.Templates.Amount.colorsSpentToCast spell) Semantics.Comparator.eq 0
}
