keyword splitSecond : Ability := {
  lexical := { lemma := "split second" },
  spelling := "split second",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.staticSpecConjunction Option.none
            [Semantics.Templates.StaticSpec.cant (Semantics.Templates.Subject.actor (Semantics.Templates.NounPhrase.bare Semantics.Templates.Predicate.isPlayer))
                (Semantics.Definitions.cast (Semantics.Templates.NounPhrase.bare Semantics.Templates.Predicate.anotherSpell)),
              Semantics.Templates.StaticSpec.cant (Semantics.Templates.Subject.actor (Semantics.Templates.NounPhrase.bare Semantics.Templates.Predicate.isPlayer))
                (Semantics.Definitions.activate
                  (Semantics.Templates.NounPhrase.allOf
                    (Semantics.Templates.Predicate.and
                      [Semantics.Templates.Predicate.abilityHead Semantics.AbilityClass.anyActivated,
                        Semantics.Templates.Predicate.non Semantics.Templates.Predicate.isManaAbility])))])
          (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisSpell (Semantics.Templates.Predicate.in_ Semantics.Templates.Location.stack)))]
}
