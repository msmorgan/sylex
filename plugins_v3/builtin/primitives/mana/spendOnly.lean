/-- "Spend this mana only to <purposes>." -/
primitive spendOnly (purposes : List SpendPurpose) : ManaRider := {
  spelling := "Spend this mana only to <purposes>.",
  body :=
    Semantics.Primitives.ManaRider.spendOnly purposes
}
