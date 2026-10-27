keyword convoke : Ability := {
  lexical := { lemma := "convoke", verbs := [{ lemma := "convoke", frames := [Monotransitive] }] },
  spelling := "convoke",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.paySubstitute Semantics.Templates.NounPhrase.thisSpell
          {
            action :=
              Semantics.Definitions.tap
                (Semantics.Templates.NounPhrase.a
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isUntapped, Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.youControl])),
            pays :=
              [Semantics.Templates.PipPaid.generic,
                Semantics.Templates.PipPaid.ofColor
                  (Semantics.Templates.ColorTerm.colorOf (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.creature))] })]
}
