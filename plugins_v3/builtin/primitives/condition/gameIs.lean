/-- "if it's <designation>": the game has it [CR#731.1]. -/
-- [CR#731.1]
primitive gameIs (designation : DesignationLabel) : Condition := {
  spelling := "if it's <designation>",
  body :=
    Semantics.Primitives.Condition.gameHas designation
}
