/-- "goad <subject>": goaded until the next turn of the controller of the goading spell or
ability, "your next turn" [CR#701.15a,109.5], as detain's. -/
action goad (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "goad", frames := [Monotransitive] }] },
  spelling := "goad <subject>",
  body :=
    gainDesignation subject goaded
        untilYourNextTurn
}
