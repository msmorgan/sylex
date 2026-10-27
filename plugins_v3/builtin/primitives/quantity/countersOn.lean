primitive countersOn (kind : CounterKind) (holder : NounPhrase) : Amount := {
  spelling := "the number of <kind> counters on <holder>",
  body :=
    Semantics.Primitives.Amount.statOf (Semantics.ProjAxis.counter kind) holder
}
