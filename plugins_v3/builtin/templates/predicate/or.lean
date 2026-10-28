/-- "<p> or <q>": some disjunct holds. -/
template or (disjuncts : List Predicate) : Predicate := {
  -- TODO(spelling): Context-dependent wording: <p> or <q>; string substitution cannot express its omitted or inflected parts.
  body := coord .or disjuncts
}
