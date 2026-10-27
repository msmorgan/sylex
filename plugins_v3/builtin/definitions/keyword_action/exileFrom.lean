action exileFrom (subject : NounPhrase) (from_ : Location) : Action := {
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  deed := "exile",
  spelling := "exile <subject> from <from_>",
  body :=
    Semantics.Templates.Action.actionMove subject from_ (Option.some Semantics.Templates.Location.exileZone) Semantics.ObservationPoint.arrival []
}
