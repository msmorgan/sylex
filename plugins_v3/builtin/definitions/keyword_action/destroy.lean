/-- "destroy <subject>" (`Definition`, [CR#701.8]). -/
action destroy (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "destroy", frames := [Monotransitive] }] },
  spelling := "destroy <subject>",
  body :=
    actionMove subject (zone Semantics.Zone.battlefield none)
      (some graveyard) Semantics.ObservationPoint.departure []
}
