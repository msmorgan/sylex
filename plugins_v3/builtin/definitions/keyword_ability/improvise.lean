keyword improvise : Ability := {
  lexical := { lemma := "improvise" },
  spelling := "improvise",
  body :=
    [ static
        (.paySubstitute thisSpell
          ⟨ tap (a (and [isUntapped, isArtifact, youControl])),
            [.generic] ⟩) ]
}
