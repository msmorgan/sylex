keyword firebending (amount : Amount) : Ability := {
  lexical := {
    lemma := "firebending",
    parameters := [[Amount]],
    verbs := [{ lemma := "firebend", past := "firebent", frames := [Intransitive] }]
  },
  spelling := "firebending <amount>",
  body :=
    [ whenever (does thisCreature (.attack none))
        (sequentially
          [ .addMana amount (.runs [[ColorOrColorless.of .red]]) [],
            establishThroughout (.manaRetention .you .thisMana) untilEndOfCombat ]) ]
}
