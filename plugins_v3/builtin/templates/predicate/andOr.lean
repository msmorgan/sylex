/-- "<p> and/or <q>": some disjunct holds, as "with flying and/or reach". -/
template andOr (disjuncts : List Predicate) : Predicate := {
  -- TODO(spelling): Context-dependent wording: <p> and/or <q>; string substitution cannot express its omitted or inflected parts.
  body := coord .andOr disjuncts
}
