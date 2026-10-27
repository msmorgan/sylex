keyword powerUp (ability : Ability) : Ability := {
  lexical := { lemma := "power-up", parameters := [[DashAbility]] },
  spelling := "power-up — <ability>",
  body :=
    [Semantics.Templates.Ability.qualified ability Option.none (Option.some Semantics.Templates.UsageLimit.oncePerObject) Option.none
        Option.none,
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.reducedBySymbols Semantics.Templates.NounPhrase.thisAbility
            (Semantics.Templates.ManaCostTerm.manaCostOf Semantics.Templates.NounPhrase.thisPermanent))
          (Semantics.Templates.Condition.did (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            Semantics.Templates.Action.enter (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.soFarThisTurn)))]
}
