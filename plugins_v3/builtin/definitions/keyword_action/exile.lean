/-- "exile <subject>" (`Definition`, [CR#701.13]). -/
action exile (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  spelling := "exile <subject>",
  body :=
    Semantics.Templates.Action.actionMove subject Semantics.Templates.Location.wherever (Option.some Semantics.Templates.Location.exileZone)
      Semantics.ObservationPoint.arrival []
}
