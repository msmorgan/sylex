action suspect (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "suspect", frames := [Monotransitive] }] },
  spelling := "suspect <subject>",
  body :=
    gainDesignation subject suspected
}
