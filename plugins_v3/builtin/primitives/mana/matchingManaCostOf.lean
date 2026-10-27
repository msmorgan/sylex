/-- "mana equal to <subject>'s mana cost" -/
primitive matchingManaCostOf (subject : NounPhrase) : ProducedMana := {
  spelling := "mana equal to <subject>'s mana cost",
  body :=
    Semantics.Primitives.ProducedMana.asPrintedCost subject
}
