keyword exhaust (ability : Ability) : Ability := {
  lexical := { lemma := "exhaust", parameters := [[DashAbility]] },
  spelling := "exhaust — <ability>",
  body :=
    [Semantics.Templates.Ability.qualified ability Option.none (Option.some Semantics.Templates.UsageLimit.oncePerObject) Option.none
        Option.none]
}
