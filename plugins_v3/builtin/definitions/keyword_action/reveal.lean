/-- "reveal <cards>" (`Definition`, [CR#701.20]). -/
action reveal (cards : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "reveal", frames := [Monotransitive] }] },
  spelling := "reveal <cards>",
  body :=
    expose reveal (Templates.Exposed.cards cards)
}
