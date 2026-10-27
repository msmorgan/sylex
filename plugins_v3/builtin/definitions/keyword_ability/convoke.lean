keyword convoke : Ability := {
  lexical := { lemma := "convoke", verbs := [{ lemma := "convoke", frames := [Monotransitive] }] },
  spelling := "convoke",
  body :=
    [ static
        (.paySubstitute thisSpell
          ⟨ tap (a (and [isUntapped, isCreature, youControl])),
            [.generic, .ofColor (.colorOf (that creature))] ⟩) ]
}
