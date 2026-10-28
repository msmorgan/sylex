/-- "<performer> flips <count> coin(s)" [CR#705.1]. -/
primitive actionFlipCoins (count : FlipScope) (call : Option FlipCall) (outcome : Option CoinFace) : Action := {
  body := Semantics.Primitives.Action.flipCoins count call outcome
}
