/-- "∞ — [Ability]" means "As long as this permanent is harnessed, it has [ability]."
[CR#702.186b] -/
keyword infinity (ability : Ability) : Ability := {
  lexical := { lemma := "∞", parameters := [[DashAbility]] },
  spelling := "∞ — <ability>",
  body :=
    [ static
        (onlyWhile (abilityGrant thisPermanent ability)
          (is thisPermanent (hasDesignation harnessed))) ]
}
