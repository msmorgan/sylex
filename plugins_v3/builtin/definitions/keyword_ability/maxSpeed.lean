keyword maxSpeed (ability : Ability) : Ability := {
  lexical := { lemma := "max speed", parameters := [[DashAbility]] },
  spelling := "max speed — <ability>",
  body :=
    [static (onlyWhile (abilityGrant this_ ability) (.compareAmt (speedOf .you) .eq 4))]
}
