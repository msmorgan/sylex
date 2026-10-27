/-- "<condition> continuously since <since>": `condition` has held from `since` until now. -/
primitive heldSince (condition : Condition) (since : GameEvent) : Condition := {
  spelling := "<condition> continuously since <since>",
  body :=
    Semantics.Primitives.Condition.heldSince condition since
}
