keyword solved (ability : Ability) : Ability := {
  lexical := { lemma := "solved", parameters := [[DashAbility]] },
  spelling := "solved — <ability>",
  body :=
    [ qualified ability none none
        (is thisCase (hasDesignation DesignationLabel.solved))
        none ]
}
