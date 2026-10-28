primitive triggered (event : GameEvent) (alternatives : List GameEvent) (while_ : Option Timing)
    (joins : List JoinedHeader) (timing : Option Timing) (limit : Option UsageLimit)
    (intervening : Option Condition) (action : Action) : Ability := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Ability.triggered event alternatives while_ joins timing limit intervening action
}
