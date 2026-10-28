/-- Exile the subject from the specified zone. [CR#701.13a] -/
action exileFrom (subject : NounPhrase) (from_ : Location) : Action := {
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  deed := "exile",
  spelling := "exile <subject> from <from_>",
  body :=
    move subject from_ (some exileZone) .arrival []
}
