/-- [CR#703.4a,502.1,702.26a] "Immediately after the untap step begins, all phased-in permanents
with phasing that the active player controls phase out, and all phased-out permanents that the
active player controlled when they phased out phase in." One row, so each permanent does one or
the other: one phased out this way is not phased in again by the same action. The active player
is read as the controller whose turn it is [CR#102.1]. The phase-in half reads the phased-out
permanent's present controller, which phasing does not change [CR#702.26d]; it differs from the
one it phased out under only if a control effect ended while it was phased out [CR#702.26f], and
reading that one needs a term for the active player (`lean-v3-active-player-term`). An Aura,
Equipment or Fortification that phased out indirectly, with the permanent it is attached to,
doesn't phase in by itself but with that permanent [CR#702.26g], so the phase-in half skips a
phased-out permanent attached to a phased-out one. That reads "indirectly" by the attachment: an
attachment that phased out directly and whose host later phased out too is skipped as well,
which the rule would phase in (`lean-v3-phasing-direct-phase-out`). That every permanent changes
at once is the engine's. -/
conferral phasingTurnBased := {

  body :=
    { scope := Semantics.Templates.Predicate.isPermanent,
      confer :=
        Semantics.Conferral.turnBased Semantics.TurnPart.untapStep
          (Semantics.Templates.Action.doIf
            (Semantics.Templates.Condition.duringPart Semantics.TurnPart.turn
              (Option.some (Semantics.Templates.NounPhrase.controllerOf Semantics.Templates.NounPhrase.this_)))
            (Semantics.Templates.Action.doIf
              (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.this_ Semantics.Templates.Predicate.isPhasedOut)
              (Semantics.Templates.Action.doIf
                (Semantics.Templates.Condition.not
                  (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.this_
                    (Semantics.Templates.Predicate.predicateAttachment Semantics.AttachmentSide.attachment Option.none
                      (Option.some
                        (Semantics.Templates.NounPhrase.a
                          (Semantics.Templates.Predicate.and
                            [Semantics.Templates.Predicate.isPermanent, Semantics.Templates.Predicate.isPhasedOut]))))))
                (Semantics.Templates.Action.setStatus Semantics.Status.phasedIn Semantics.Templates.NounPhrase.this_))
              (Option.some
                (Semantics.Templates.Action.doIf
                  (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.this_
                    (Semantics.Templates.Predicate.hasKeyword (Semantics.Templates.KeywordTerm.theKeyword Semantics.Definitions.phasing.label)))
                  (Semantics.Templates.Action.setStatus Semantics.Status.phasedOut Semantics.Templates.NounPhrase.this_))))) }
}
