/-- "Decayed" means "This creature can't block" and "When this creature attacks, sacrifice it at end
of combat." [CR#702.147a] -/
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
        (atNext endOfCombat
          (sacrifice thisPermanent))]
}
