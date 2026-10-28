keyword undaunted : Ability := {
  lexical := { lemma := "undaunted" },
  spelling := "undaunted",
  body :=
    [static
        (costShift thisSpell
          (costShiftGeneric (down (forEach 1 isOpponent))
            none))]
}
