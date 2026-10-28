/-- "return <subject> to the battlefield under <who>'s control with N <kind> counters on it" -/
template returnToBattlefieldWithCounters (subject who : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action := {
  -- TODO(spelling): return <subject> to the battlefield under <who>'s control with N <kind> counters on it; wording varies with amount, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    zoneChange subject battlefield wherever
      [under who,
        withCounters amount (printedKind kind) Semantics.EntryCounterMark.fresh]
}
