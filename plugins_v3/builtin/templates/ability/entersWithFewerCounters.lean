/-- "<subject> enters with N fewer <kind> counters on it" -/
template entersWithFewerCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : StaticSpec := {
  -- TODO(spelling): <subject> enters with N fewer <kind> counters on it; wording varies with amount, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    enters subject
      [withCounters amount (printedKind kind) .fewer]
}
