/-- "A creature with defender can't attack." [CR#702.3b] -/
keyword defender : Ability := {
  lexical := { lemma := "defender" },
  spelling := "defender",
  body :=
    [static
        (cant (actor thisCreature)
          (attack none))]
}
