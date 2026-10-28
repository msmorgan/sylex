/-- Losing counters is removal from the named player. -/
template loseCounters (kind : Option CounterKindSource) (amount : Option Amount)
    (agent : NounPhrase := you) : Action := {
  -- TODO(spelling): Losing counters is removal from the named player.
  body :=
    actionRemoveCounters (Option.map exactlyOf amount) kind agent
}
