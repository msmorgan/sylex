keyword forecast (ability : Ability) : Ability := {
  lexical := { lemma := "forecast", parameters := [[DashAbility]] },
  spelling := "forecast — <ability>",
  body :=
    [Semantics.Templates.Ability.qualified ability
        (Option.some
          (Semantics.Templates.Timing.duringPart Semantics.TurnPart.upkeep
            (Option.some (Semantics.Templates.NounPhrase.ownerOf Semantics.Templates.NounPhrase.this_))))
        (Option.some Semantics.Templates.UsageLimit.oncePerTurn) Option.none (Option.some Semantics.Zone.hand)]
}
