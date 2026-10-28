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
