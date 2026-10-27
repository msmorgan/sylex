keyword improvise : Ability := {
  lexical := { lemma := "improvise" },
  spelling := "improvise",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.paySubstitute Semantics.Templates.NounPhrase.thisSpell
          {
            action :=
              Semantics.Definitions.tap
                (Semantics.Templates.NounPhrase.a
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isUntapped, Semantics.Templates.Predicate.isArtifact, Semantics.Templates.Predicate.youControl])),
            pays := [Semantics.Templates.PipPaid.generic] })]
}
