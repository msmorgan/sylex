/-- "<kind> counters on it" as an entering copy has them -/
primitive withEntryCounters (amount : Amount) (kind : CounterKind) (mark : EntryCounterMark) :
    CopyExcept := {
  -- TODO(spelling): <kind> counters on it; wording varies with amount, mark, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.CopyExcept.entersWithCounters amount kind mark
}
