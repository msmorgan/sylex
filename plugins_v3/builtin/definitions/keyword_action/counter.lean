/-- "counter <subject>": one labeled stack-to-graveyard move (`Definition`, [CR#701.6a]). -/
action counter (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "counter", frames := [Monotransitive] }] },
  spelling := "counter <subject>",
  body :=
    actionMove subject (zone Semantics.Zone.stack none)
      (some (zone Semantics.Zone.graveyard none)) Semantics.ObservationPoint.arrival []
}
