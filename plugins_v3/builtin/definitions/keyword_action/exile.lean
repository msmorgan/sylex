/-- "exile <subject>" (`Definition`, [CR#701.13]). -/
action exile (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  spelling := "exile <subject>",
  body :=
    .move subject .anyLocation exileZone
        .arrival []
}
