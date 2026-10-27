/-- "Ripple N" with its definition [CR#702.60a]: "When you cast this spell, you may reveal the top
N cards of your library. If you do, you may cast any of those cards with the same name as this
spell without paying their mana costs, then put all revealed cards not cast this way on the bottom
of your library in any order." -/
keyword ripple (amount : Amount) : Ability := {
  lexical := { lemma := "ripple", parameters := [[Amount]] },
  spelling := "ripple <amount>",
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.thisSpell))
        (Semantics.Templates.Action.may
          (Semantics.Definitions.reveal
            (Semantics.Templates.NounPhrase.slice Semantics.PileEnd.top amount
              (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.you)))
          (Option.some
            (Semantics.Templates.Action.sequentially
              [Semantics.Definitions.castWithoutPaying
                  (Semantics.Templates.NounPhrase.nounPhraseSomeOf (Semantics.Templates.SliceCount.sliceCountCounted Semantics.Templates.Quantity.anyNumber)
                    (Option.some
                      (Semantics.Templates.Predicate.shares Semantics.QualitySort.cardName Semantics.Agreement.share
                        Semantics.Templates.NounPhrase.this_))
                    (Semantics.Templates.NounPhrase.those Semantics.NounWord.card))
                  Option.none,
                Semantics.Templates.Action.actionMove
                  (Semantics.Templates.NounPhrase.allOf
                    (Semantics.Templates.Predicate.and
                      [Semantics.Templates.Predicate.isCard,
                        Semantics.Templates.Predicate.was (Semantics.Definitions.reveal Semantics.Templates.NounPhrase.which)
                          (Option.some Semantics.Templates.LookbackSpan.thisWay),
                        Semantics.Templates.Predicate.not
                          (Semantics.Templates.Predicate.was (Semantics.Definitions.cast Semantics.Templates.NounPhrase.which)
                            (Option.some Semantics.Templates.LookbackSpan.thisWay))]))
                  Semantics.Templates.Location.wherever (Option.some Semantics.Templates.Location.onBottom)
                  Semantics.ObservationPoint.arrival [Semantics.Templates.MoveRider.inOrder Semantics.Arrangement.anyOrder]])))]
}
