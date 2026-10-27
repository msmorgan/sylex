action exileFrom (subject : NounPhrase) (from_ : Location) : Action := {
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  deed := "exile",
  spelling := "exile <subject> from <from_>",
  body :=
    .move subject from_
        exileZone .arrival []
}
