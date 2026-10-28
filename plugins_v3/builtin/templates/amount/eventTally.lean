/-- How often, or how much, `subj` did `action` over `within`: a tally always reads a written
span. -/
template eventTally (subj : Subject) (action : Action) (within : LookbackSpan)
    (mode : TallyMode) : Amount := {
  -- TODO(spelling): The mode selects an occurrence count or accumulated magnitude, requiring
  -- different historical clauses.
  body :=
    tally { subj := subj, action := action, within := some within } mode
}
