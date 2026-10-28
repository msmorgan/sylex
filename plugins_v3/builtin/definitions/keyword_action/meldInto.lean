/-- "meld <subject> into <name>" (`Definition`, [CR#701.42]). -/
action meldInto (subject : NounPhrase) (into : String) : Action := {
  lexical := { verbs := [{ lemma := "meld", frames := [(TransitiveWithPP ["into"])] }] },
  deed := "meld",
  spelling := "meld <subject> into <into>",
  body :=
    move subject wherever (some battlefield)
      .arrival [entersMelded into]
}
