/-- Losing counters is removal from the named player. -/
template loseCounters (kind : Option CounterKindSource) (amount : Option Amount)
    (agent : NounPhrase := you) : Action := {
  -- TODO(spelling): Losing counters is removal from the named player.
  body :=
    Semantics.Templates.Action.actionRemoveCounters (Option.map Semantics.Templates.Quantity.exactlyOf amount) kind agent
}
