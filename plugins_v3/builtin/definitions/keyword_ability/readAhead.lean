keyword readAhead : Ability := {
  lexical := { lemma := "read ahead" },
  spelling := "read ahead",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.cant
            (Semantics.Templates.Subject.actor
              (Semantics.Templates.NounPhrase.allOf
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.chapterAbilityOf Semantics.Templates.NounPhrase.thisSaga,
                    Semantics.Templates.Predicate.not
                      (Semantics.Templates.Predicate.predicateCompare [Semantics.ProjAxis.chapter] Semantics.Comparator.eq
                        (Semantics.Templates.Amount.countersOn Semantics.Definitions.loreCounter
                          Semantics.Templates.NounPhrase.thisSaga))])))
            Semantics.Templates.Action.trigger)
          (Semantics.Templates.Condition.did (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisSaga)
            Semantics.Templates.Action.enter (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.soFarThisTurn)))]
}
