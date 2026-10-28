/-- "with N <kind> counters on it" -/
primitive withCounters (amount : Amount) (kind : CounterKindSource) (mark : EntryCounterMark) :
    MoveRider := {
  -- TODO(spelling): The mark selects ordinary, additional, or fewer counters, changing the
  -- placement of amount and kind.
  body :=
    Semantics.Primitives.MoveRider.withCounters amount kind mark
}
