/-- A counter kind printed on the card: "+1/+1 counter". -/
primitive printedKind (kind : CounterKind) : CounterKindSource := {
  -- TODO(spelling): Context-dependent wording: +1/+1 counter; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.CounterKindSource.printed kind
}
