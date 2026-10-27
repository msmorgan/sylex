/-- "<action>. Do this only once each turn.": its ability triggers only if its source's controller
has not yet taken the action this turn [CR#603.2h]. -/
-- [CR#603.2h]
template doOnlyOnceEachTurn (action : Action) : Action := {
  spelling := "<action>. Do this only once each turn.",
  body :=
    Semantics.Templates.Action.limited action Semantics.Templates.UsageLimit.oncePerTurn
}
