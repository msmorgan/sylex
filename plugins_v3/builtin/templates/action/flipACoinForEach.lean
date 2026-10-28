/-- "flip a coin for each <each>" -/
template flipACoinForEach (each : NounPhrase) (call : Option FlipCall := none)
    (outcome : Option CoinFace := none) : Action := {
  -- TODO(spelling): flip a coin for each <each>; wording varies with call, outcome, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    actionFlipCoins (per each) call outcome
}
