template gain (subject : NounPhrase) (ability : Ability)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    establish
      (characteristicChange subject [characteristicEditAddedAbilities [ability]]) duration
}
