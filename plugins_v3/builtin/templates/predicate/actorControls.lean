/-- "… <performer> controls" -/
template actorControls : Predicate := {
  -- TODO(spelling): Context-dependent wording: … <performer> controls; string substitution cannot express its omitted or inflected parts.
  body :=
    hasPossessor .controller theActor
}
