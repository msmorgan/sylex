/-- "may <body>. When they do, <trigger>": a reflexive trigger on the choice. -/
template offerWhen (body : Action) (triggerBody : Action) : Action := {
  -- TODO(spelling): Context-dependent wording: may <body>. When they do, <trigger>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.actionTriggerReflexively
      (Semantics.Templates.Action.withContinuation Semantics.ContinuationPolicy.optional body Option.none Option.none) Option.none
      triggerBody
}
