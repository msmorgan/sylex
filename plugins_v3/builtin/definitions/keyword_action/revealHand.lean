/-- "<performer> reveals their hand" (`Definition`, [CR#701.20]). -/
action revealHand : Action := {
  lexical := { verbs := [{ lemma := "reveal", frames := [Monotransitive] }] },
  deed := "reveal",
  spelling := "reveal your hand",
  body :=
    expose .reveal
      (zone (handOf actor))
}
