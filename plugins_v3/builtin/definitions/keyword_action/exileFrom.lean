action exileFrom (subject : NounPhrase) (from_ : Location) : Action := {
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  deed := "exile",
  spelling := "exile <subject> from <from_>",
  body :=
    actionMove subject from_ (some exileZone) Semantics.ObservationPoint.arrival []
}
