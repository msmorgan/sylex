/-- "one mana of any type <subject> produced" -/
primitive ofTypeProducedBy (subject : NounPhrase) : ProducedMana := {
  spelling := "one mana of any type <subject> produced",
  body :=
    Semantics.Primitives.ProducedMana.producedByEvent subject
}
