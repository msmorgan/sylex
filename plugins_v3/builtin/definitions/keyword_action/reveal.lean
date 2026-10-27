/-- "reveal <cards>" (`Definition`, [CR#701.20]). -/
action reveal (cards : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "reveal", frames := [Monotransitive] }] },
  spelling := "reveal <cards>",
  body :=
    Semantics.Templates.Action.expose Semantics.ExposeVerb.reveal (Semantics.Templates.Exposed.cards cards)
}
