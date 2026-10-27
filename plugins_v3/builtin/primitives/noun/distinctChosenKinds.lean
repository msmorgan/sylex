/-- "N different counters from among <menu>" -/
primitive distinctChosenKinds (menu : List CounterKind) : CounterKindSource := {
  spelling := "N different counters from among <menu>",
  body :=
    Semantics.Primitives.CounterKindSource.distinctChosen menu
}
