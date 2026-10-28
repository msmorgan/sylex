/-- The accumulated magnitude of historical action events. -/
template eventSum (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount := {
  -- TODO(spelling): Include or omit the clauses supplied by timing.
  body :=
    tally
      { subj := subj, action := action, within := some (Templates.LookbackSpan.within within timing) }
      .quantity
}
