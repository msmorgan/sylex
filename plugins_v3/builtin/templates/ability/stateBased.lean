template stateBased (when : Condition) (then_ : Action) : Conferral := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Conferral.stateBased when then_
}
