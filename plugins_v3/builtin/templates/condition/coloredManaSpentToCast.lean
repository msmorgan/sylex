/-- "if colored mana was spent to cast <spell>" -/
template coloredManaSpentToCast (spell : NounPhrase) : Condition := {
  spelling := "if colored mana was spent to cast <spell>",
  body :=
    Semantics.Templates.Condition.compareAmt (Semantics.Templates.Amount.colorsSpentToCast spell) Semantics.Comparator.atLeast 1
}
