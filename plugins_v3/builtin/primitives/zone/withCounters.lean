/-- "with N <kind> counters on it" -/
primitive withCounters (amount : Amount) (kind : CounterKindSource) (mark : EntryCounterMark) :
    MoveRider := {
  -- TODO(spelling): with N <kind> counters on it; wording varies with amount, mark, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.MoveRider.withCounters amount kind mark
}
