keyword reinforce (amount : Amount) (cost : Cost) : Ability := {
  lexical := { lemma := "reinforce", parameters := [[Amount, DashCost]] },
  spelling := "reinforce <amount>—<cost>",
  body :=
    [Semantics.Templates.Ability.activated
        (Semantics.Templates.Cost.compound [cost, Semantics.Templates.Cost.perform (Semantics.Definitions.discard Semantics.Templates.NounPhrase.this_)])
        (Semantics.Templates.Action.putCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          (Semantics.Templates.NounPhrase.target Semantics.Templates.Predicate.isCreature))]
}
