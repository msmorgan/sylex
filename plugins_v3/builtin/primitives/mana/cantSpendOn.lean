/-- "You can't spend this mana to <purposes>." -/
primitive cantSpendOn (purposes : List SpendPurpose) : ManaRider := {
  spelling := "You can't spend this mana to <purposes>.",
  body :=
    Semantics.Primitives.ManaRider.spendNotOn purposes
}
