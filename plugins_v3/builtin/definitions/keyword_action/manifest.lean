/-- "<performer> manifests <subject>" (`Definition`, [CR#701.40a,708.2,708.2a]). -/
action manifest (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "manifest", frames := [Monotransitive] }] },
  spelling := "manifest <subject>",
  body :=
    sequentially
      [turnFaceDown subject faceDownCreature,
        actionMove subject wherever (some battlefield)
          .arrival [faceDownAs faceDownCreature]]
}
