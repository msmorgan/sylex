action heal (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "heal", frames := [Monotransitive] }] },
  spelling := "heal <subject>",
  body :=
    clearDamage subject
}
