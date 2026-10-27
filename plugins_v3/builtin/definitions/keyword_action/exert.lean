action exert (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "exert", frames := [Monotransitive] }] },
  spelling := "exert <subject>",
  body :=
    Semantics.Templates.Action.skipUntap subject 1
}
