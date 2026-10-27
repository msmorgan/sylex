/-- "create <count> <token> tapped and attacking" -/
template createTappedAttacking (count : Amount) (token : CharacteristicBundle)
    (defender : Option NounPhrase := none) : Action := {
  -- TODO(spelling): create <count> <token>s that are tapped and attacking [<defender>]; string
  -- substitution cannot branch on optional fields or supply contextual agreement and morphology.
  body :=
    create count token [entersAs .tapped, entersAttacking defender]
}
