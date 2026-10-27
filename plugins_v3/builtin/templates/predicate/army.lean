/-- An Army the performer controls. -/
template army : Predicate := {
  spelling := "Army creature you control",
  body :=
    Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.hasSubtype Subtype.army, isCreature, actorControls]
}
