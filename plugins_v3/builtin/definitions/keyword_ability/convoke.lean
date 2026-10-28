keyword convoke : Ability := {
  lexical := { lemma := "convoke", verbs := [{ lemma := "convoke", frames := [Monotransitive] }] },
  spelling := "convoke",
  body :=
    [static
        (paySubstitute thisSpell
          {
            action :=
              tap
                (a
                  (and
                    [isUntapped, isCreature, youControl])),
            pays :=
              [generic,
                ofColor
                  (colorOf (that creature))] })]
}
