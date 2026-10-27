/-- "Cumulative upkeep [cost]" with its reminder text [CR#702.24a]. -/
keyword cumulativeUpkeep (cost : Cost) : Ability := {
  lexical := {
    lemma := "cumulative upkeep",
    parameters := [[Cost], [DashNonManaCost]],
    nouns := [{ lemma := "cumulative upkeep", countability := [Mass] }]
  },
  -- TODO(spelling): cumulative upkeep [mana cost] / cumulative upkeep—[verbal cost]; separator
  -- and terminal period depend on the Cost structure.
  body :=
    [Semantics.Templates.Ability.at_
        (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.upkeep (Option.some Semantics.Templates.NounPhrase.you)
          (Semantics.Templates.TurnRef.eachTime Option.none))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.ageCounter)
              Semantics.Templates.NounPhrase.thisPermanent,
            Semantics.Templates.Action.doUnlessKeywordCost (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thisPermanent)
              (Semantics.Templates.Cost.scaled cost
                (Semantics.Templates.Amount.times 1
                  (Semantics.Templates.Amount.countersOn Semantics.Definitions.ageCounter Semantics.Templates.NounPhrase.thisPermanent)))
              Semantics.Templates.NounPhrase.thisPermanent Semantics.Definitions.cumulativeUpkeep.label])
        [] [] Option.none Option.none Option.none
        (Option.some (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent Semantics.Templates.Predicate.isPermanent))]
}
