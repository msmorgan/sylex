/-- The Actor creates a double-faced Token with the specified faces and entry riders.
Token characteristics are defined by the creating effect [CR#111.3]; the Incubator is a
predefined double-faced Token [CR#111.10i]. -/
template createDoubleFacedToken (front back : CharacteristicBundle)
    (riders : List MoveRider := []) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    createObject 1 (token (doubleFaced front back) riders)
      none
}
