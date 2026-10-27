/-- "a counter of the chosen kind" -/
primitive thatKind : CounterKindSource := {
  spelling := "a counter of the chosen kind",
  body :=
    Semantics.Primitives.CounterKindSource.bound
}
