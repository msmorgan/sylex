/-- "exile <subject> with N <kind> counters on it" (`Definition`, [CR#701.13]). -/
action exileWithCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action := {
  -- TODO(spelling): exile <subject> with <amount> <kind> counters on it/them; string substitution
  -- cannot select counter number or pronoun agreement.
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  deed := "exile",
  body :=
    Semantics.Templates.Action.actionMove subject Semantics.Templates.Location.wherever (Option.some Semantics.Templates.Location.exileZone)
      Semantics.ObservationPoint.arrival
      [Semantics.Templates.MoveRider.withCounters amount (Semantics.Templates.CounterKindSource.printedKind kind) Semantics.EntryCounterMark.fresh]
}
