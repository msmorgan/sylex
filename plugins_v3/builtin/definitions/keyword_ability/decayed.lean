keyword decayed : Ability := {
  lexical := { lemma := "decayed" },
  spelling := "decayed",
  body :=
    [ static (cant thisCreature (.block .any)),
      when (does thisCreature (.attack none))
        (atNext .endOfCombat (sacrifice thisPermanent)) ]
}
