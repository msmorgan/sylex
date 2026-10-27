keyword webSlinging (cost : Cost) : Ability := {
  lexical := { lemma := "web-slinging", parameters := [[Cost]] },
  spelling := "web-slinging <cost>",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_
          (Option.some
            (Semantics.Templates.Cost.compound
              [cost,
                Semantics.Templates.Cost.perform
                  (Semantics.Templates.Action.returnToHand
                    (Semantics.Templates.NounPhrase.a
                      (Semantics.Templates.Predicate.and
                        [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isTapped,
                          Semantics.Templates.Predicate.youControl])))])))]
}
