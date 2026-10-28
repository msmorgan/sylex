/-- "Intimidate": "This creature can't be blocked except by artifact creatures and/or creatures
that share a color with it." [CR#702.13b] -/
keyword intimidate : Ability := {
  lexical := { lemma := "intimidate" },
  spelling := "intimidate",
  body :=
    [ static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature,
          andOr [isArtifact, shares color share thisCreature]])))) ]
}
