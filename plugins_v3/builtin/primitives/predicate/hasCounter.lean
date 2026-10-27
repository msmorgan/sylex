/-- "with a counter on it", or "with a <kind> counter on it". -/
primitive hasCounter (kind : Option CounterKind := none) : Predicate := {
  -- TODO(spelling): with a counter on it; wording varies with kind, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Predicate.hasCounters kind
}
