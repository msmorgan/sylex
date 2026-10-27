keyword skulk : Ability := {
  lexical := { lemma := "skulk" },
  spelling := "skulk",
  body :=
    [ static
        (cant (.by (allOf (and [isCreature, hasPower .greater (powerOf thisCreature)])))
          (.block thisCreature)) ]
}
