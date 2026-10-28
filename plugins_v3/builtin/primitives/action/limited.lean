/-- Apply a usage limit to the action. -/
primitive limited (action : Action) (limit : UsageLimit) : Action := {
  body :=
    Semantics.Primitives.Action.limited action limit
}
