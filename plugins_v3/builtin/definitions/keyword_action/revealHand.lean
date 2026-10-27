/-- "<performer> reveals their hand" (`Definition`, [CR#701.20]). -/
action revealHand : Action := {
  lexical := { verbs := [{ lemma := "reveal", frames := [Monotransitive] }] },
  deed := "reveal",
  spelling := "reveal your hand",
  body :=
    Semantics.Templates.Action.expose Semantics.ExposeVerb.reveal
      (Semantics.Templates.Exposed.exposedZone (Semantics.Templates.Location.handOf Semantics.Templates.NounPhrase.actor))
}
