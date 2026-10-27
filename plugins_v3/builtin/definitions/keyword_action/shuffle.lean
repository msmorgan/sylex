/-- A bare "shuffle": the performer's own library (`Definition`, [CR#701.24a]). -/
action shuffle : Action := {
  lexical := { verbs := [{ lemma := "shuffle", frames := [Intransitive, Monotransitive, (TransitiveWithPP ["into"])] }] },
  spelling := "shuffle",
  body :=
    Semantics.Templates.Action.actionShuffle (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.actor)
}
