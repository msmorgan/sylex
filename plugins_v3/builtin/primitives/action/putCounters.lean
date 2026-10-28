/-- Put the specified amount and kind of counters on the subject. -/
primitive putCounters (amount : Amount) (kind : CounterKindSource) (on : NounPhrase) :
    Action := {
  spelling := "put <amount> <kind> counters on <on>",
  body :=
    Semantics.Primitives.Action.putCounters amount kind on
}
