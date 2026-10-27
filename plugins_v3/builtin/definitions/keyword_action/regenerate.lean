/-- A resolving regeneration action installs a single-use shield for this turn;
only its eventual application carries the Regenerate label (`Definition`, [CR#701.19a,701.19c]). -/
action regenerate (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "regenerate", frames := [Intransitive, Monotransitive] }] },
  deed := none,
  spelling := "regenerate <subject>",
  body :=
    Semantics.Templates.Action.establish
      (Semantics.Templates.StaticSpec.replace
        (Semantics.Templates.WouldClause.would Semantics.Templates.Subject.any (Semantics.Definitions.destroy subject))
        (Semantics.Templates.Replacement.instead (Semantics.Definitions.regeneration subject)) Option.none
        Semantics.ReplUse.nextTimeOnly Option.none)
      (Option.some Semantics.Templates.Duration.thisTurn)
}
