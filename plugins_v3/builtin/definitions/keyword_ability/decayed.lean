keyword decayed : Ability := {
  lexical := { lemma := "decayed" },
  spelling := "decayed",
  body :=
    [static
        (cant (actor thisCreature)
          (block any)),
      when
        (does (actor thisCreature)
          (attack none))
        (atNext .endOfCombat
          (sacrifice thisPermanent))]
}
