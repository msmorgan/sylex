/-- The number of occurrences of a historical action clause. -/
template eventCount (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount := {
  -- TODO(spelling): The number of occurrences of a historical action clause.
  body :=
    Semantics.Templates.Amount.amountTally
      { subj := subj, action := action, within := Option.some (Semantics.Templates.LookbackSpan.within within timing) }
      Semantics.TallyMode.events
}
