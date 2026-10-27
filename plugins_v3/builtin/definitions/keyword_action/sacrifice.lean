/-- "sacrifice <subject>" (`Definition`, [CR#701.21]). -/
action sacrifice (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "sacrifice", frames := [Monotransitive] }] },
  spelling := "sacrifice <subject>",
  body :=
    Semantics.Templates.Action.actionMove subject (Semantics.Templates.Location.zone Semantics.Zone.battlefield Option.none)
      (Option.some Semantics.Templates.Location.graveyard) Semantics.ObservationPoint.departure []
}
