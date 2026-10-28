action exert (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exert", frames := [Monotransitive] }] },
  spelling := "exert <subject>",
  body :=
    skipUntap subject 1
}
