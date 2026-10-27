/-- "counter <subject>": one labeled stack-to-graveyard move (`Definition`, [CR#701.6a]). -/
action counter (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "counter", frames := [Monotransitive] }] },
  spelling := "counter <subject>",
  body :=
    .move subject (.zone .stack none)
        (some (.zone .graveyard none))
        .arrival []
}
