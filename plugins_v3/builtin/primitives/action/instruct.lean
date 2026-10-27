/-- "<performer> <body>": the handoff (`Action.instruct`), to a player or a permanent. -/
primitive instruct (performer : NounPhrase) (body : Action) : Action := {
  spelling := "<performer> <body>",
  body :=
    Semantics.Primitives.Action.instruct performer body
}
