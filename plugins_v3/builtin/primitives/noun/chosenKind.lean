/-- "your choice of a <kind> or <kind> counter" -/
primitive chosenKind (menu : List CounterKind) : CounterKindSource := {
  body :=
    Semantics.Primitives.CounterKindSource.chosen menu
}
