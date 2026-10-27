/-- "destroy <subject>" (`Definition`, [CR#701.8]). -/
action destroy (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "destroy", frames := [Monotransitive] }] },
  spelling := "destroy <subject>",
  body :=
    Semantics.Templates.Action.actionMove subject (Semantics.Templates.Location.zone Semantics.Zone.battlefield Option.none)
      (Option.some Semantics.Templates.Location.graveyard) Semantics.ObservationPoint.departure []
}
