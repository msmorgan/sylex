/-- The accumulated magnitude of historical action events. -/
template eventSum (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount := {
  -- TODO(spelling): The accumulated magnitude of historical action events.
  body :=
    amountTally
      { subj := subj, action := action, within := some (LookbackSpan.within within timing) }
      Semantics.TallyMode.quantity
}
