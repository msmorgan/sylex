keyword improvise : Ability := {
  lexical := { lemma := "improvise" },
  spelling := "improvise",
  body :=
    [static
        (paySubstitute thisSpell
          {
            action :=
              tap
                (a
                  (and
                    [isUntapped, isArtifact, youControl])),
            pays := [generic] })]
}
