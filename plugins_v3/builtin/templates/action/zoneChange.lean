/-- The Zone Change every put and return Template writes: `subject` moves from `from_` to
`destination` with `riders` [CR#400.7]. "Put" and "return" name no deed, so the two verbs expand
to the same move; each Template keeps its own wording. The written origin, not the verb, decides
the Observation Point (`Location.observedOnDeparture`): observation follows the pattern, not an
individual occurrence's origin, so a move from the battlefield, or from a list naming it, is
observed on its departure, and "from anywhere" is not a leaves-the-battlefield trigger
[CR#603.6c]. -/
-- [CR#400.7]
-- [CR#603.6c]
template zoneChange (subject : NounPhrase) (destination : Location)
    (from_ : Location) (riders : List MoveRider) : Action := {
  -- TODO(spelling): Put; wording varies with destination, from_, riders, subject, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    actionMove subject from_ (some destination)
      (if Semantics.Location.observedOnDeparture from_ = true then Semantics.ObservationPoint.departure
      else Semantics.ObservationPoint.arrival)
      riders
}
