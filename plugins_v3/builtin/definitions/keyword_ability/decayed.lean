keyword decayed : Ability := {
  lexical := { lemma := "decayed" },
  spelling := "decayed",
  body :=
    [static
        (cant (actor thisCreature)
          (block NounPhrase.any)),
      when
        (does (actor thisCreature)
          (attack none))
        (atNext Semantics.TurnPart.endOfCombat
          (sacrifice thisPermanent))]
}
