/-- A resolving regeneration action installs a single-use shield for this turn;
only its eventual application carries the Regenerate label (`Definition`, [CR#701.19a,701.19c]). -/
action regenerate (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "regenerate", frames := [Intransitive, Monotransitive] }] },
  deed := none,
  spelling := "regenerate <subject>",
  body :=
    establish
      (replace
        (would any (Semantics.Definitions.destroy subject))
        (instead (Semantics.Definitions.regeneration subject)) none
        nextTimeOnly none)
      (some thisTurn)
}
