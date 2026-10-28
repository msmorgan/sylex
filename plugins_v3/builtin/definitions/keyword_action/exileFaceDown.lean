/-- "exile <subject> face down" (`Definition`, [CR#701.13,406.3]). -/
action exileFaceDown (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  deed := "exile",
  spelling := "exile <subject> face down",
  body :=
    actionMove subject wherever (some exileZone)
      Semantics.ObservationPoint.arrival [faceDownAs exiledFaceDown]
}
