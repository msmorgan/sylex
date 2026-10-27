keyword flanking : Ability := {
  lexical := { lemma := "flanking" },
  spelling := "flanking",
  body :=
    [ whenever
        (does (.by (a (and [isCreature, not (.hasKeyword flanking)])))
          (.block thisCreature))
        (get (that creature) (.down 1) (.down 1) untilEndOfTurn) ]
}
