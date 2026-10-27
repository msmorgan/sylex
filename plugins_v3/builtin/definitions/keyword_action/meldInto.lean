/-- "meld <subject> into <name>" (`Definition`, [CR#701.42]). -/
action meldInto (subject : NounPhrase) (into : String) : Action := {
  lexical := { verbs := [{ lemma := "meld", frames := [(TransitiveWithPP ["into"])] }] },
  deed := "meld",
  spelling := "meld <subject> into <into>",
  body :=
    Semantics.Templates.Action.actionMove subject Semantics.Templates.Location.wherever (Option.some Semantics.Templates.Location.battlefield)
      Semantics.ObservationPoint.arrival [Semantics.Templates.MoveRider.moveRiderEntersMelded into]
}
