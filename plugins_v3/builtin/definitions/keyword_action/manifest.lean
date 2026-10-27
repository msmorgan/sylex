/-- "<performer> manifests <subject>" (`Definition`, [CR#701.40a,708.2,708.2a]). -/
action manifest (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "manifest", frames := [Monotransitive] }] },
  spelling := "manifest <subject>",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.turnFaceDown subject Semantics.Templates.FaceDown.faceDownCreature,
        Semantics.Templates.Action.actionMove subject Semantics.Templates.Location.wherever (Option.some Semantics.Templates.Location.battlefield)
          Semantics.ObservationPoint.arrival [Semantics.Templates.MoveRider.faceDownAs Semantics.Templates.FaceDown.faceDownCreature]]
}
