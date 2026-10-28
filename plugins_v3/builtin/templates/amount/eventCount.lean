/-- The number of occurrences of a historical action clause. -/
template eventCount (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount := {
  -- TODO(spelling): The number of occurrences of a historical action clause.
  body :=
    amountTally
      { subj := subj, action := action, within := some (LookbackSpan.within within timing) }
      Semantics.TallyMode.events
}
