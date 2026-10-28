template createDoubleFacedToken (front back : CharacteristicBundle)
    (riders : List MoveRider := []) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    createObject 1 (token (doubleFaced front back) riders)
      none
}
