/-- "return <subject> to the battlefield under <who>'s control with N <kind> counters on it" -/
template returnToBattlefieldWithCounters (subject who : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action := {
  spelling := "return <subject> to the battlefield under <who>'s control with <amount> <kind> counters on it",
  body :=
    zoneChange subject battlefield wherever
      [under who,
        withCounters amount (printedKind kind) .fresh]
}
