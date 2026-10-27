/-- "counter <subject>": one labeled stack-to-graveyard move (`Definition`, [CR#701.6a]). -/
action counter (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "counter", frames := [Monotransitive] }] },
  spelling := "counter <subject>",
  body :=
    Semantics.Templates.Action.actionMove subject (Semantics.Templates.Location.zone Semantics.Zone.stack Option.none)
      (Option.some (Semantics.Templates.Location.zone Semantics.Zone.graveyard Option.none)) Semantics.ObservationPoint.arrival []
}
