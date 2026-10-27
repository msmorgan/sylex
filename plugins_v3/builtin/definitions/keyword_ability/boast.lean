keyword boast (ability : Ability) : Ability := {
  lexical := {
    lemma := "boast",
    parameters := [[DashAbility]],
    verbs := [{ lemma := "boast", frames := [Intransitive] }]
  },
  spelling := "boast — <ability>",
  body :=
    [ .qualified ability none oncePerTurn
        (is thisCreature (did which (.attack none) soFarThisTurn))
        none ]
}
