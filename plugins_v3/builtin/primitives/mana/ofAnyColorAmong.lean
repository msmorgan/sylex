/-- "one mana of any of <colors>" -/
primitive ofAnyColorAmong (colors : List Color) : ProducedMana := {
  spelling := "one mana of any of <colors>",
  body :=
    Semantics.Primitives.ProducedMana.amongWritten colors
}
