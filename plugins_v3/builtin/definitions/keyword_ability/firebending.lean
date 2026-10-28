/-- "Firebending N" means "Whenever this creature attacks, add N {R}. Until end of combat, you don't
lose this mana as steps and phases end." [CR#702.189a] -/
keyword firebending (amount : Amount) : Ability := {
  lexical := {
    lemma := "firebending",
    parameters := [[Amount]],
    verbs := [{ lemma := "firebend", past := "firebent", frames := [Intransitive] }]
  },
  spelling := "firebending <amount>",
  body :=
    [whenever
        (does (actor thisCreature)
          (attack none))
        (sequentially
          [addMana amount
              (manaOf [[.of .red]]) [],
            establishThroughout
              (manaRetention you thisMana)
              untilEndOfCombat])]
}
