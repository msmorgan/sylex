primitive actionFlipCoins (count : Semantics.FlipScope) (call : Option Semantics.FlipCall) (outcome : Option Semantics.CoinFace) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.flipCoins count call outcome
}
