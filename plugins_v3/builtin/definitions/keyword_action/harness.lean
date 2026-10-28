/-- Harness a permanent: if it is not harnessed, it becomes harnessed. [CR#701.64a] -/
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
