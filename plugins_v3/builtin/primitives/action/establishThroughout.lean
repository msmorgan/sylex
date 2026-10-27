/-- "<spec> <duration>": a clause holding for a stated duration. -/
primitive establishThroughout (spec : StaticSpec) (duration : Duration) : Action := {
  spelling := "<spec> <duration>",
  body :=
    Semantics.Primitives.Action.establish spec (Option.some duration)
}
