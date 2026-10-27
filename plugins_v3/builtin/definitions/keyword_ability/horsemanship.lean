keyword horsemanship : Ability := {
  lexical := { lemma := "horsemanship" },
  spelling := "horsemanship",
  body :=
    [ static
        (cant (.by (allOf (and [isCreature, not (.hasKeyword horsemanship)])))
          (.block thisCreature)) ]
}
