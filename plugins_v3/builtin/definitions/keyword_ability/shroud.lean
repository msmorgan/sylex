/-- "Shroud": "This permanent or player can't be the target of spells or abilities"
[CR#702.18a], written over its holder (`this_`). -/
keyword shroud : Ability := {
  lexical := { lemma := "shroud" },
  spelling := "shroud",
  body :=
    [static
        (cant Subject.any (target this_))]
}
