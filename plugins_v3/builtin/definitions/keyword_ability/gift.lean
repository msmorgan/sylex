keyword gift (promise : Action) : Ability := {
  lexical := { lemma := "gift", parameters := [[Quality (None)]], nouns := ["gift"] },
  -- TODO(spelling): Gift a card / gift a tapped Fish; the promised Action must select a licensed
  -- noun phrase, not print an action clause.
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.addedCost (Semantics.Templates.Cost.perform (Semantics.Templates.Action.choose Semantics.Templates.NounPhrase.anOpponent))
          Bool.true Semantics.PayTimes.once),
      Semantics.Templates.Ability.byCardClass
        (Semantics.Templates.Ability.when
          (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.enter)
          (Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.the Semantics.Templates.Predicate.isChosenPlayer) promise) [] []
          Option.none Option.none Option.none
          (Option.some (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.gift.label Semantics.Templates.NounPhrase.this_)))
        (Semantics.Templates.Ability.spell
          (Semantics.Templates.Action.doIf
            (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.gift.label Semantics.Templates.NounPhrase.this_)
            (Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.the Semantics.Templates.Predicate.isChosenPlayer) promise)))]
}
