/-- "with a counter on it", or "with a <kind> counter on it". -/
primitive hasCounter (kind : Option CounterKind := none) : Predicate := {
  -- TODO(spelling): Include or omit the clauses supplied by kind.
  body :=
    Semantics.Primitives.Predicate.hasCounters kind
}
