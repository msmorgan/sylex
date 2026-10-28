/-- "mana of the last noted type" -/
primitive ofTheLastNotedType (subject : NounPhrase) : ProducedMana := {
  spelling := "mana of the last noted type",
  body :=
    Semantics.Primitives.ProducedMana.lastNoted subject
}
