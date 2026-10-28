/-- Allows the same mode to be chosen more than once. [CR#700.2d] -/
primitive repeatable : ModeChange := {
  spelling := "You may choose the same mode more than once.",
  body :=
    Semantics.Primitives.ModeChange.repeatable
}
