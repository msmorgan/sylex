action harness (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "harness", frames := [Monotransitive] }] },
  spelling := "harness <subject>",
  body :=
    doIf
        (not
          (is subject
            (hasDesignation harnessed)))
        (gainDesignation subject harnessed)
        none
}
