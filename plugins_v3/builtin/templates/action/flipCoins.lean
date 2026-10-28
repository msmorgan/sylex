/-- "flip <count> coins [and win/lose] [showing <outcome>]" -/
template flipCoins (count : Amount := anyAmount)
    (call : Option FlipCall := none) (outcome : Option CoinFace := none) : Action := {
  -- TODO(spelling): Context-dependent wording: flip <count> coins [and win/lose] [showing <outcome>]; string substitution cannot express its omitted or inflected parts.
  body :=
    actionFlipCoins (flipScopeCount count) call outcome
}
