/-- "If this spell's <keyword> cost was paid, <do X to> the permanent this spell becomes at the
beginning of the next end step": a Resolution Delayed Trigger, created as the permanent is put
onto the battlefield as the spell resolves [CR#608.3g]. -/
-- [CR#608.3g]
primitive resolutionDelayedTrigger (does : Action) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: If this spell's <keyword> cost was paid, <do X to> the permanent this spell becomes at the beginning of the next end step; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.StaticSpec.resolutionDelayedTrigger does
}
