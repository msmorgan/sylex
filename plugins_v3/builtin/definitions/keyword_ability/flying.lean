/-- "Flying" with its definition, a static ability: "This creature can't be blocked except by
creatures with flying and/or reach." [CR#702.9b] -/
keyword flying : Ability := {
  lexical := { lemma := "flying" },
  spelling := "flying",
  body :=
    [ static (cant any (block thisCreature)
        (except := byObjects (bare (and [isCreature,
          andOr [hasKeyword flying, hasKeyword reach]])))) ]
}
