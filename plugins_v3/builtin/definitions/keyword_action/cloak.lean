/-- "<performer> cloaks <subject>" (`Definition`, [CR#701.58a]). -/
action cloak (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "cloak", frames := [Monotransitive] }] },
  spelling := "cloak <subject>",
  body :=
    sequentially
      [turnFaceDown subject faceDownWardedCreature,
        actionMove subject wherever (some battlefield)
          .arrival [faceDownAs faceDownWardedCreature]]
}
