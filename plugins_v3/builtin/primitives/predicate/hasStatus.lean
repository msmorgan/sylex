primitive hasStatus (status : Status) : Predicate := {
  spelling := "<status>",
  body :=
    Semantics.Primitives.Predicate.hasStatus status
}
