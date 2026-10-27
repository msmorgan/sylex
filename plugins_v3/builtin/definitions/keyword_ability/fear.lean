/-- "Fear": "This creature can't be blocked except by artifact creatures and/or black
creatures." [CR#702.36b] -/
keyword fear : Ability := {
  lexical := { lemma := "fear" },
  spelling := "fear",
  body :=
    [ static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature, andOr [isArtifact, hasColor .black]])))) ]
}
