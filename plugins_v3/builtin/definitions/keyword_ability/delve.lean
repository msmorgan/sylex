keyword delve : Ability := {
  lexical := { lemma := "delve" },
  spelling := "delve",
  body :=
    [ static
        (.paySubstitute thisSpell
          ⟨ exile (a (cardIn yourGraveyard)),
            [.generic] ⟩) ]
}
