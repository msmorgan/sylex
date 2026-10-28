primitive actionFlipCoins (count : FlipScope) (call : Option FlipCall) (outcome : Option CoinFace) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.flipCoins count call outcome
}
