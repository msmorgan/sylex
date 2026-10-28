primitive hasPriority (player : NounPhrase) : Condition := {
  spelling := "<player> has priority",
  body := Semantics.Primitives.Condition.hasPriority player
}
