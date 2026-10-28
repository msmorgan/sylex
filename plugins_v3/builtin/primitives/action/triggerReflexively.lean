/-- "<body>. When you do, <trigger>" [CR#603.12] -/
-- [CR#603.12]
primitive triggerReflexively (body triggerBody : Action)
    (event : Option GameEvent := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by event.
  body := Semantics.Primitives.Action.triggerReflexively body event triggerBody
}
