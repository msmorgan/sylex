keyword outlast (cost : Cost) : Ability := {
  lexical := { lemma := "outlast", parameters := [[Cost]] },
  spelling := "outlast <cost>",
  body :=
    [ activatedOnlyDuring (.compound [cost, .tapSymbol])
        (putCounters 1 p1p1Counter thisCreature)
        asSorcery ]
}
