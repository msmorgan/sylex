/-- "flip a coin for each <each>" -/
template flipACoinForEach (each : NounPhrase) (call : Option FlipCall := none)
    (outcome : Option CoinFace := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by call, outcome.
  body :=
    actionFlipCoins (per each) call outcome
}
