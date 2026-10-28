keyword delve : Ability := {
  lexical := { lemma := "delve" },
  spelling := "delve",
  body :=
    [static
        (paySubstitute thisSpell
          {
            action :=
              exile
                (a (cardIn yourGraveyard)),
            pays := [generic] })]
}
