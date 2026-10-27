keyword skulk : Ability := {
  lexical := { lemma := "skulk" },
  spelling := "skulk",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant
          (Semantics.Templates.Subject.by_
            (Semantics.Templates.NounPhrase.allOf
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCreature,
                  Semantics.Templates.Predicate.hasPower Semantics.Comparator.greater
                    (Semantics.Templates.Amount.powerOf Semantics.Templates.NounPhrase.thisCreature)])))
          (Semantics.Templates.Action.block Semantics.Templates.NounPhrase.thisCreature))]
}
