primitive isPhasedOut : Predicate := {
  spelling := "phased out",
  body :=
    Semantics.Primitives.Predicate.hasStatus Semantics.Status.phasedOut
}
