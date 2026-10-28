/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a]. -/
action manifestPlacement (subject : NounPhrase) : Action := {
  -- TODO(spelling): manifest <subject>; this bench fragment models placement only, so printing the
  -- complete keyword would overstate its meaning.
  lexical := { verbs := [{ lemma := "manifest", frames := [Monotransitive] }] },
  deed := "manifest",
  body :=
    actionMove subject wherever (some battlefield)
      Semantics.ObservationPoint.arrival []
}
