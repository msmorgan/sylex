/-- "Paradigm" represents two spell abilities [CR#702.192a]: "If this is the first time a spell
you control with this spell's name has resolved this game, at the beginning of each of your
precombat main phases for the rest of the game, create a copy of this object in exile. You may
cast the copy without paying its mana cost" and "Exile this spell." It is the first time when no
other spell you controlled with this spell's name resolved earlier this game. -/
keyword paradigm : Ability := {
  lexical := { lemma := "paradigm" },
  spelling := "paradigm",
  body :=
    [Semantics.Templates.Ability.spell
        (Semantics.Templates.Action.doIf
          (Semantics.Templates.Condition.not
            (Semantics.Templates.Condition.did Semantics.Templates.Subject.any
              (Semantics.Definitions.resolve
                (Semantics.Templates.NounPhrase.a
                  (Semantics.Templates.Predicate.and
                    [Semantics.Templates.Predicate.isSpell, Semantics.Templates.Predicate.isOther, Semantics.Templates.Predicate.youControl,
                      Semantics.Templates.Predicate.shares Semantics.QualitySort.cardName Semantics.Agreement.share
                        Semantics.Templates.NounPhrase.thisSpell])))
              (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.thisGame)))
          (Semantics.Templates.Action.delayWithin (Semantics.Templates.GameEvent.beginningOfYour Semantics.TurnPart.firstMain)
            Semantics.Templates.Duration.restOfGame
            (Semantics.Templates.Action.sequentially
              [Semantics.Templates.Action.copy (Semantics.CopySort.inZone Semantics.Zone.exile) Semantics.Templates.NounPhrase.this_ 1 [],
                Semantics.Templates.Action.may
                  (Semantics.Definitions.cast (Semantics.Templates.NounPhrase.that Semantics.NounWord.copy) Option.none
                    Semantics.Templates.PlayPayment.withoutPaying)]))),
      Semantics.Templates.Ability.spell (Semantics.Definitions.exile Semantics.Templates.NounPhrase.thisSpell)]
}
