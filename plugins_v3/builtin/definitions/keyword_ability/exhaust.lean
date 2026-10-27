keyword exhaust (ability : Ability) : Ability := {
  lexical := { lemma := "exhaust", parameters := [[DashAbility]] },
  spelling := "exhaust — <ability>",
  body :=
    [.qualified ability none oncePerObject none none]
}
