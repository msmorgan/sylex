/-- "exile <subject> from <from_> face down" (`Definition`, [CR#701.13,406.3]): a move that names
its origin, so an ability whose cost or effect makes it functions only in that zone
[CR#113.6m]. -/
action exileFaceDownFrom (subject : NounPhrase) (from_ : Location) : Action := {
  lexical := { verbs := [{ lemma := "exile", frames := [Monotransitive] }] },
  deed := "exile",
  spelling := "exile <subject> from <from_> face down",
  body :=
    Semantics.Templates.Action.actionMove subject from_ (Option.some Semantics.Templates.Location.exileZone) Semantics.ObservationPoint.arrival
      [Semantics.Templates.MoveRider.faceDownAs Semantics.Templates.FaceDown.exiledFaceDown]
}
