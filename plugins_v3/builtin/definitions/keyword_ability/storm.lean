/-- "Storm": "When you cast this spell, copy it for each other spell that was cast before it this
turn. If the spell has any targets, you may choose new targets for any of the copies."
[CR#702.40a] -/
keyword storm : Ability := {
  lexical := { lemma := "storm" },
  spelling := "storm",
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisSpell))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.copy Semantics.CopySort.fromStack Semantics.Templates.NounPhrase.thisSpell
              (Semantics.Templates.Amount.eventCount
                (Semantics.Definitions.cast
                  (Semantics.Templates.NounPhrase.a
                    (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isSpell, Semantics.Templates.Predicate.isOther])))
                (Semantics.Templates.Subject.actor (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer))
                Semantics.Templates.Duration.earlierThisTurn)
              [],
            Semantics.Templates.Action.may
              (Semantics.Templates.Action.chooseNewTargets
                (Semantics.Templates.NounPhrase.nounPhrasePro (Semantics.Reach.word Semantics.NounWord.copy) Semantics.Plurality.many))])]
}
