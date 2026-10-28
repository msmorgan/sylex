/-- "<subject> enters with an additional N <kind> counters on it" -/
template entersWithAdditionalCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : StaticSpec := {
  -- TODO(spelling): <subject> enters with an additional N <kind> counters on it; wording varies with amount, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    enters subject
      [withCounters amount (printedKind kind)
          .additional]
}
