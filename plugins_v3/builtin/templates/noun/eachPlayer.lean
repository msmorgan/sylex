/-- "each player" -/
template eachPlayer : NounPhrase := {
  spelling := "each player",
  body :=
    Semantics.Templates.NounPhrase.each Semantics.Templates.Predicate.isPlayer
}
