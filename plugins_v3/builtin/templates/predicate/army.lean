/-- An Army the performer controls. -/
template army : Predicate := {
  spelling := "Army creature you control",
  body :=
    .and [.hasSubtype Subtype.army, isCreature, actorControls]
}
