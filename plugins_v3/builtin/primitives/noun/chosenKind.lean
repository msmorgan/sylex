/-- "your choice of a <kind> or <kind> counter" -/
primitive chosenKind (menu : List CounterKind) : CounterKindSource := {
  -- TODO(spelling): Context-dependent wording: your choice of a <kind> or <kind> counter; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.CounterKindSource.chosen menu
}
