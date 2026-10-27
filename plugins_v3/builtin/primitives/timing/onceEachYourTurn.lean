/-- "only once during each of your turns" -/
primitive onceEachYourTurn : UsageLimit := {
  spelling := "only once during each of your turns",
  body :=
    Semantics.Primitives.UsageLimit.mk 1 Semantics.Period.yourTurn
}
