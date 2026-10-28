/-- "flip <count> coins [and win/lose] [showing <outcome>]" -/
template flipCoins (count : Amount := anyAmount)
    (call : Option FlipCall := none) (outcome : Option CoinFace := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by call, outcome.
  body :=
    actionFlipCoins (count count) call outcome
}
