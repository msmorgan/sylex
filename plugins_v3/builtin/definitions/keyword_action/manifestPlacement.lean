/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a]. -/
action manifestPlacement (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "manifest", frames := [Monotransitive] }] },
  deed := "manifest",
  body :=
    move subject wherever (some battlefield)
      .arrival []
}
