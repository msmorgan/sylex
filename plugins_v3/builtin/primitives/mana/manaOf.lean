/-- "Add <runs>": one of the listed runs of mana symbols. -/
primitive manaOf (runs : List ProducedRun) : ProducedMana := {
  spelling := "Add <runs>",
  body :=
    Semantics.Primitives.ProducedMana.runs runs
}
