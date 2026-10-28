/-- "<body>. When <event> this way, <trigger>" [CR#603.12] -/
-- [CR#603.12]
template triggerThisWay (body : Action) (event : GameEvent) (triggerBody : Action) :
    Action := {
  -- TODO(spelling): Context-dependent wording: <body>. When <event> this way, <trigger>; string substitution cannot express its omitted or inflected parts.
  body := triggerReflexively body triggerBody (some event)
}
