/-- "tap <subject>" (`Definition`, [CR#701.26a]). -/
action tap (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "tap", past := "tapped", gerund := "tapping", frames := [Monotransitive] }] },
  spelling := "tap <subject>",
  body :=
    Semantics.Templates.Action.setStatus Semantics.Status.tapped subject
}
