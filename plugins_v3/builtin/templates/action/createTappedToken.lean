/-- "create <count> tapped <token>" -/
template createTappedToken (token : CharacteristicBundle) (count : Amount := 1) :
    Action := {
  -- TODO(spelling): create a tapped <token> / create <count> tapped <token>s; string substitution
  -- cannot branch on optional fields or supply contextual agreement and morphology.
  body :=
    create count token [entersAs .tapped]
}
