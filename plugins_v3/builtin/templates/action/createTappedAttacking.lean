/-- "create <count> <token> tapped and attacking" -/
template createTappedAttacking (count : Amount) (token : CharacteristicBundle)
    (defender : Option NounPhrase := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by defender.
  body :=
    create count token [entersAs tapped, entersAttacking defender]
}
