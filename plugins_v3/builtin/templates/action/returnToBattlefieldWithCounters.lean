/-- "return <subject> to the battlefield under <who>'s control with N <kind> counters on it" -/
template returnToBattlefieldWithCounters (subject who : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action := {
  -- TODO(spelling): return <subject> to the battlefield under <who>'s control with N <kind> counters on it; wording varies with amount, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Templates.Action.zoneChange subject Semantics.Templates.Location.battlefield Semantics.Templates.Location.wherever
      [Semantics.Templates.MoveRider.under who,
        Semantics.Templates.MoveRider.withCounters amount (Semantics.Templates.CounterKindSource.printedKind kind) Semantics.EntryCounterMark.fresh]
}
