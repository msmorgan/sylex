/-- "Echo [cost]": "At the beginning of your upkeep, if this permanent came under your control
since the beginning of your last upkeep, sacrifice it unless you pay [cost]." [CR#702.30a] It came
under your control since then unless you have controlled it continuously since then. The payment
is of this permanent's echo cost, which a replacement or a trigger on paying it reads. -/
keyword echo (cost : Cost) : Ability := {
  lexical := { lemma := "echo", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): echo [mana cost] / echo—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [Semantics.Templates.Ability.at_ (Semantics.Templates.GameEvent.beginningOfYour Semantics.TurnPart.upkeep)
        (Semantics.Templates.Action.doUnlessKeywordCost (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thisPermanent) cost
          Semantics.Templates.NounPhrase.thisPermanent Semantics.Definitions.echo.label)
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.not
            (Semantics.Templates.Condition.heldSince
              (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent Semantics.Templates.Predicate.youControl)
              (Semantics.Templates.GameEvent.beginningOf Semantics.TurnPart.upkeep (Option.some Semantics.Templates.NounPhrase.you)
                (Semantics.Templates.TurnRef.turnRefPrevious (Option.some Semantics.Templates.NounPhrase.you))))))]
}
