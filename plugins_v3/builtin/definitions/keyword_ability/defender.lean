keyword defender : Ability := {
  lexical := { lemma := "defender" },
  spelling := "defender",
  body :=
    [static
        (cant (actor thisCreature)
          (attack none))]
}
