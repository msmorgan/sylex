keyword surge (cost : Cost) : Ability := {
  lexical := { lemma := "surge", parameters := [[Cost]] },
  spelling := "surge <cost>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_ (Option.some cost))
          (Semantics.Templates.Condition.did
            (Semantics.Templates.Subject.actor
              (Semantics.Templates.NounPhrase.or
                [Semantics.Templates.NounPhrase.you, Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isTeammate]))
            (Semantics.Definitions.cast (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.anotherSpell))
            (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.soFarThisTurn)))]
}
