/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a]. -/
action manifestPlacement (subject : NounPhrase) : Action := {
  -- TODO(spelling): manifest <subject>; this bench fragment models placement only, so printing the
  -- complete keyword would overstate its meaning.
  lexical := { verbs := [{ lemma := "manifest", frames := [Monotransitive] }] },
  deed := "manifest",
  body :=
    Semantics.Templates.Action.actionMove subject Semantics.Templates.Location.wherever (Option.some Semantics.Templates.Location.battlefield)
      Semantics.ObservationPoint.arrival []
}
