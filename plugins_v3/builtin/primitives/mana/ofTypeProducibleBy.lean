/-- "one mana of any type <subject> could produce" -/
primitive ofTypeProducibleBy (subject : NounPhrase) : ProducedMana := {
  spelling := "one mana of any type <subject> could produce",
  body :=
    Semantics.Primitives.ProducedMana.couldProduce subject
}
