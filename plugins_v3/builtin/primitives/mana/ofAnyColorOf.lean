/-- "one mana of any of <subject>'s colors" -/
primitive ofAnyColorOf (subject : NounPhrase) : ProducedMana := {
  spelling := "one mana of any of <subject>'s colors",
  body :=
    Semantics.Primitives.ProducedMana.amongColorsOf subject
}
