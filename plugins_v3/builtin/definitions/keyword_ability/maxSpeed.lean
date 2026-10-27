keyword maxSpeed (ability : Ability) : Ability := {
  lexical := { lemma := "max speed", parameters := [[DashAbility]] },
  spelling := "max speed — <ability>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile (Semantics.Templates.StaticSpec.abilityGrant Semantics.Templates.NounPhrase.this_ ability)
          (Semantics.Templates.Condition.compareAmt (Semantics.Templates.Amount.speedOf Semantics.Templates.NounPhrase.you) Semantics.Comparator.eq
            4))]
}
