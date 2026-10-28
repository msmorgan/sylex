keyword undaunted : Ability := {
  lexical := { lemma := "undaunted" },
  spelling := "undaunted",
  body :=
    [static
        (costShift thisSpell
          (generic (down (forEach 1 isOpponent))
            none))]
}
