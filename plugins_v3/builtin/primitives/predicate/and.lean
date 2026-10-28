/-- "<p> and <q>": every conjunct holds. -/
primitive and (conjuncts : List Predicate) : Predicate := {
  -- TODO(spelling): Context-dependent wording: <p> and <q>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Predicate.coord Semantics.Coordinator.and conjuncts
}
