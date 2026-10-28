/-- "<who> loses all [<kind>] counters" -/
template loseAllCounters (kind : Option CounterKindSource)
    (agent : NounPhrase := you) : Action := {
  -- TODO(spelling): Context-dependent wording: <who> loses all [<kind>] counters; string substitution cannot express its omitted or inflected parts.
  body :=
    loseCounters kind none agent
}
