/-- "exile <subject> with N <kind> counters on it" (`Definition`, [CR#701.13]). -/
action exileWithCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action := {
  -- TODO(spelling): exile <subject> with <amount> <kind> counters on it/them; string substitution
  -- cannot select counter number or pronoun agreement.
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  deed := "exile",
  body :=
    actionMove subject wherever (some exileZone)
      Semantics.ObservationPoint.arrival
      [withCounters amount (printedKind kind) Semantics.EntryCounterMark.fresh]
}
