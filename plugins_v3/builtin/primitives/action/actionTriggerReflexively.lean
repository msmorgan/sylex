primitive actionTriggerReflexively (body : Semantics.Action) (event : Option Semantics.GameEvent) (trigger : Semantics.Action) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.triggerReflexively body event trigger
}
