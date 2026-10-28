/-- "sacrifice <subject>" (`Definition`, [CR#701.21]). -/
action sacrifice (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "sacrifice", frames := [Monotransitive] }] },
  spelling := "sacrifice <subject>",
  body :=
    move subject (zone battlefield none)
      (some graveyard) .departure []
}
