keyword delve : Ability := {
  lexical := { lemma := "delve" },
  spelling := "delve",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.paySubstitute Semantics.Templates.NounPhrase.thisSpell
          {
            action :=
              Semantics.Definitions.exile
                (Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.cardIn Semantics.Templates.Location.yourGraveyard)),
            pays := [Semantics.Templates.PipPaid.generic] })]
}
