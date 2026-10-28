/-- Put the specified amount and kind of counters on the subject. -/
primitive putCounters (amount : Amount) (kind : CounterKindSource) (on : NounPhrase) :
    Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.putCounters amount kind on
}
