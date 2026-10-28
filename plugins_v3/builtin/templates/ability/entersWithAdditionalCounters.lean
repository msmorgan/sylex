/-- "<subject> enters with an additional N <kind> counters on it" -/
template entersWithAdditionalCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : StaticSpec := {
  spelling := "<subject> enters with an additional <amount> <kind> counters on it",
  body :=
    enters subject
      [withCounters amount (printedKind kind)
          additional]
}
