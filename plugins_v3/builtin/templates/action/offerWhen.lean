/-- "may <body>. When they do, <trigger>": a reflexive trigger on the choice. -/
template offerWhen (body : Action) (triggerBody : Action) : Action := {
  spelling := "You may <body>. When you do, <triggerBody>.",
  body :=
    triggerReflexively (withContinuation .optional body none none) triggerBody none
}
