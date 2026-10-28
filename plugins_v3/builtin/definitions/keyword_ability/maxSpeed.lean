/-- "Max speed — [Ability]" means "As long as your speed is 4, this object has '[Ability].'"
[CR#702.178a] -/
keyword maxSpeed (ability : Ability) : Ability := {
  lexical := { lemma := "max speed", parameters := [[DashAbility]] },
  spelling := "max speed — <ability>",
  body :=
    [static
        (onlyWhile (abilityGrant this_ ability)
          (compareAmt (speedOf you) eq
            4))]
}
