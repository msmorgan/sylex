/-- "Menace": "This creature can't be blocked except by two or more creatures."
[CR#702.111b] -/
keyword menace : Ability := {
  lexical := { lemma := "menace" },
  spelling := "menace",
  body :=
    [ static (cant any (block thisCreature)
        (except := byObjects (counted (orMore 2) isCreature))) ]
}
