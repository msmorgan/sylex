keyword boast (ability : Ability) : Ability := {
  lexical := {
    lemma := "boast",
    parameters := [[DashAbility]],
    verbs := [{ lemma := "boast", frames := [Intransitive] }]
  },
  spelling := "boast — <ability>",
  body :=
    [qualified ability none (some oncePerTurn)
        (some
          (is thisCreature
            (did (actor which)
              (attack none)
              (some (within soFarThisTurn)))))
        none]
}
