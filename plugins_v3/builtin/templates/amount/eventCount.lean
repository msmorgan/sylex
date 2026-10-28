/-- The number of occurrences of a historical action clause. -/
template eventCount (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount := {
  -- TODO(spelling): Include or omit the clauses supplied by timing.
  body :=
    tally
      { subj := subj, action := action, within := some (Templates.LookbackSpan.within within timing) }
      events
}
