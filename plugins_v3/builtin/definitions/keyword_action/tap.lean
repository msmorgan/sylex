/-- "tap <subject>" (`Definition`, [CR#701.26a]). -/
action tap (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "tap", past := "tapped", gerund := "tapping", frames := [Monotransitive] }] },
  spelling := "tap <subject>",
  body :=
    setStatus Semantics.Status.tapped subject
}
