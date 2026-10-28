/-- "<kind> counters on it" as an entering copy has them -/
primitive withEntryCounters (amount : Amount) (kind : CounterKind) (mark : EntryCounterMark) :
    CopyExcept := {
  body :=
    Semantics.Primitives.CopyExcept.entersWithCounters amount kind mark
}
