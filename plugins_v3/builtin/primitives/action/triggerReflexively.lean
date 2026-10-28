/-- "<body>. When you do, <trigger>" [CR#603.12] -/
-- [CR#603.12]
primitive triggerReflexively (body triggerBody : Action)
    (event : Option GameEvent := none) : Action := {
  -- TODO(spelling): Context-dependent wording: <body>. When you do, <trigger>; string substitution cannot express its omitted or inflected parts.
  body := Semantics.Primitives.Action.triggerReflexively body event triggerBody
}
