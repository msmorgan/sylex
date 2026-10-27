/-- "Encore [cost]": "[Cost], Exile this card from your graveyard: For each opponent, create a
token that's a copy of this card that attacks that opponent this turn if able. The tokens gain
haste. Sacrifice them at the beginning of the next end step. Activate only as a sorcery."
[CR#702.141a] -/
keyword encore (cost : Cost) : Ability := {
  lexical := { lemma := "encore", parameters := [[Cost]] },
  spelling := "encore <cost>",
  body :=
    [Semantics.Templates.Ability.activatedOnlyDuring
        (Semantics.Templates.Cost.compound
          [cost,
            Semantics.Templates.Cost.perform
              (Semantics.Definitions.exileFrom Semantics.Templates.NounPhrase.thisCard Semantics.Templates.Location.yourGraveyard)])
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.doForEach Semantics.Templates.NounPhrase.eachOpponent
              (Semantics.Templates.Action.sequentially
                [Semantics.Definitions.create 1 (Semantics.Templates.TokenSpec.copyOf Semantics.Templates.NounPhrase.this_),
                  Semantics.Templates.Action.establishThroughout
                    (Semantics.Templates.StaticSpec.must (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thatToken)
                      (Semantics.Templates.Action.attack (Option.some (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.player))))
                    Semantics.Templates.Duration.thisTurn]),
            Semantics.Templates.Action.gain Semantics.Templates.NounPhrase.thoseTokens
              (Semantics.Templates.keyword Semantics.Definitions.haste.label),
            Semantics.Templates.Action.atNext Semantics.TurnPart.endStep
              (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thoseTokens)])
        Semantics.Templates.Timing.asSorcery]
}
