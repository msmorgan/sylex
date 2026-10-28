/-- "untap <subject>" (`Definition`, [CR#701.26b]). -/
action untap (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "untap", past := "untapped", gerund := "untapping", frames := [Monotransitive, Intransitive] }] },
  spelling := "untap <subject>",
  body :=
    setStatus Semantics.Status.untapped subject
}
