/-- "teammate": a teammate of yours. -/
template isTeammate : Predicate := {
  spelling := "teammate",
  body :=
    Semantics.Templates.Predicate.isTeammateOf Semantics.Templates.NounPhrase.you
}
