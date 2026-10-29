/-- "<agent> loses <amount> [<kind>] counters": losing counters is removal from the named player.
"Loses all counters" is `loseAllCounters`. -/
template loseCounters (kind : Option CounterKindSource) (amount : Amount)
    (agent : NounPhrase := you) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by kind.
  body :=
    actionRemoveCounters (some (exactlyOf amount)) kind agent
}
