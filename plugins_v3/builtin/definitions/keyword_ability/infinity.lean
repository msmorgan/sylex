keyword infinity (ability : Ability) : Ability := {
  lexical := { lemma := "∞", parameters := [[DashAbility]] },
  spelling := "∞ — <ability>",
  body :=
    [ static
        (onlyWhile (abilityGrant thisPermanent ability)
          (is thisPermanent (hasDesignation harnessed))) ]
}
