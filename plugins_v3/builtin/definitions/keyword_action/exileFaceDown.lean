/-- "exile <subject> face down" (`Definition`, [CR#701.13,406.3]). -/
action exileFaceDown (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  deed := "exile",
  spelling := "exile <subject> face down",
  body :=
    Semantics.Templates.Action.actionMove subject Semantics.Templates.Location.wherever (Option.some Semantics.Templates.Location.exileZone)
      Semantics.ObservationPoint.arrival [Semantics.Templates.MoveRider.faceDownAs Semantics.Templates.FaceDown.exiledFaceDown]
}
