/-- "When/Whenever/At <event>, <action>" [CR#603.1]. -/
primitive triggered (event : GameEvent) (alternatives : List GameEvent) (while_ : Option Timing)
    (joins : List JoinedHeader) (timing : Option Timing) (limit : Option UsageLimit)
    (intervening : Option Condition) (action : Action) : Ability := {
  body := Semantics.Primitives.Ability.triggered event alternatives while_ joins timing limit intervening action
}
