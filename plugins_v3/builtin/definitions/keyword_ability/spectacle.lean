keyword spectacle (cost : Cost) : Ability := {
  lexical := { lemma := "spectacle", parameters := [[Cost]] },
  spelling := "spectacle <cost>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_ (Option.some cost))
          (Semantics.Templates.Condition.did (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.anOpponent)
            (Semantics.Templates.Action.loseLife Semantics.Templates.Amount.anyAmount)
            (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.soFarThisTurn)))]
}
