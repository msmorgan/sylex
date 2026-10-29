/-- "<ability> triggers by <cause>": `triggers` with what made the ability trigger, a cause
[CR#603.2]. -/
-- [CR#603.2]
template triggersBy (ability cause : NounPhrase) : GameEvent := {
  -- TODO(spelling): Word the cause clause.
  body :=
    causes (bySource cause)
      (does (actor ability) trigger)
}
