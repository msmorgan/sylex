/-- "if no mana was spent to cast <spell>" -/
template noManaSpentToCast (spell : NounPhrase) : Condition := {
  spelling := "if no mana was spent to cast <spell>",
  body :=
    Semantics.Templates.Condition.compareAmt (Semantics.Templates.Amount.manaValueSpentToCast spell) Semantics.Comparator.eq 0
}
