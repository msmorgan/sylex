primitive abilityTriggered (event : Semantics.GameEvent) (alternatives : List Semantics.GameEvent) (while_ : Option Semantics.Timing) (joins : List Semantics.JoinedHeader) (timing : Option Semantics.Timing) (limit : Option Semantics.UsageLimit) (intervening : Option Semantics.Condition) (action : Semantics.Action) : Semantics.Ability := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Ability.triggered event alternatives while_ joins timing limit intervening action
}
