/-- "if no mana was spent to cast <spell>" -/
template noManaSpentToCast (spell : NounPhrase) : Condition := {
  spelling := "if no mana was spent to cast <spell>",
  body :=
    compareAmt (manaValueSpentToCast spell) Semantics.Comparator.eq 0
}
