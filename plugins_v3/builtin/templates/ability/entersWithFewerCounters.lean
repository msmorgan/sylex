/-- "<subject> enters with N fewer <kind> counters on it" -/
template entersWithFewerCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : StaticSpec := {
  spelling := "<subject> enters with <amount> fewer <kind> counters on it",
  body :=
    enters subject
      [withCounters amount (printedKind kind) fewer]
}
