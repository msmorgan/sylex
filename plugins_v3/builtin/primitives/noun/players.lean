/-- "a <predicate> player" -/
primitive players (predicate : Predicate) : ChoiceDomain := {
  spelling := "a <predicate> player",
  body :=
    Semantics.Primitives.ChoiceDomain.players predicate
}
