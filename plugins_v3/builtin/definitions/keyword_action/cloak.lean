/-- "<performer> cloaks <subject>" (`Definition`, [CR#701.58a]). -/
action cloak (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "cloak", frames := [Monotransitive] }] },
  spelling := "cloak <subject>",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.turnFaceDown subject Semantics.Templates.FaceDown.faceDownWardedCreature,
        Semantics.Templates.Action.actionMove subject Semantics.Templates.Location.wherever (Option.some Semantics.Templates.Location.battlefield)
          Semantics.ObservationPoint.arrival [Semantics.Templates.MoveRider.faceDownAs Semantics.Templates.FaceDown.faceDownWardedCreature]]
}
