/-- "… <performer> controls" -/
template actorControls : Predicate := {
  -- TODO(spelling): The controlling Actor must be recovered from the enclosing instruction; there
  -- is no actor parameter.
  body :=
    hasPossessor .controller theActor
}
