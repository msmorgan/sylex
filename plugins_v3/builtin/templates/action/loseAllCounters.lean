/-- "<who> loses all [<kind>] counters" -/
template loseAllCounters (kind : Option CounterKindSource)
    (agent : NounPhrase := you) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by kind.
  body :=
    loseCounters kind none agent
}
