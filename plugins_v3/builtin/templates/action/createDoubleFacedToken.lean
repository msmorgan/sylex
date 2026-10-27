template createDoubleFacedToken (front back : CharacteristicBundle)
    (riders : List MoveRider := []) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Templates.Action.createObject 1 (Semantics.Templates.CreationSpec.token (Semantics.Templates.TokenSpec.tokenSpecDoubleFaced front back) riders)
      Option.none
}
