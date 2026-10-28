/-- A counter kind printed on the card: "+1/+1 counter". -/
primitive printedKind (kind : CounterKind) : CounterKindSource := {
  spelling := "<kind>",
  body :=
    Semantics.Primitives.CounterKindSource.printed kind
}
