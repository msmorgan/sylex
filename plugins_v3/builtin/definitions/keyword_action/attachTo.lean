/-- "attach <subject> to <host>" (`Definition`, [CR#701.3a]). -/
action attachTo (subject host : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "attach", frames := [(TransitiveWithPP ["to"])] }, { lemma := "unattach", frames := [Monotransitive, (TransitiveWithPP ["from"])] }] },
  deed := "attach",
  spelling := "attach <subject> to <host>",
  body :=
    attachment .attached subject (some host)
}
