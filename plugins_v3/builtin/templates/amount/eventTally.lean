/-- How often, or how much, `subj` did `action` over `within`: a tally always reads a written
span. -/
template eventTally (subj : Subject) (action : Action) (within : LookbackSpan)
    (mode : TallyMode) : Amount := {
  -- TODO(spelling): How often, or how much, `subj` did `action` over `within`: a tally always reads a written span.
  body :=
    Semantics.Templates.Amount.amountTally { subj := subj, action := action, within := Option.some within } mode
}
