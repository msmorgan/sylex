/-- "an <class_> ability" -/
primitive hasAbilityOf (class_ : AbilityClass) : DeckTrait := {
  spelling := "an <class_> ability",
  body :=
    Semantics.Primitives.DeckTrait.hasAbilityOf class_
}
