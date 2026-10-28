/-- "<p> and <q>": every conjunct holds. -/
template and (conjuncts : List Predicate) : Predicate := {
  -- TODO(spelling): Context-dependent wording: <p> and <q>; string substitution cannot express its omitted or inflected parts.
  body := coord .and conjuncts
}
