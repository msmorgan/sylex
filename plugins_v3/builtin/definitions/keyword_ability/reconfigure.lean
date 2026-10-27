/-- Reconfigure [CR#702.151a]: attaching is the `attachTo` Definition; unattaching moves the
Equipment away and is no attaching [CR#701.3d], so it is the bare attachment change and names no
deed. -/
keyword reconfigure (cost : Cost) : Ability := {
  lexical := { lemma := "reconfigure", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): reconfigure [mana cost] / reconfigure—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
  body :=
    [Semantics.Templates.Ability.activatedOnlyDuring cost
        (Semantics.Definitions.attachTo Semantics.Templates.NounPhrase.thisPermanent
          (Semantics.Templates.NounPhrase.target
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isOther, Semantics.Templates.Predicate.youControl])))
        Semantics.Templates.Timing.asSorcery,
      Semantics.Templates.Ability.abilityActivated cost (Semantics.Templates.Action.unattach Semantics.Templates.NounPhrase.thisPermanent)
        (Option.some Semantics.Templates.Timing.asSorcery) Option.none
        (Option.some
          (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent
            (Semantics.Templates.Predicate.predicateAttachment Semantics.AttachmentSide.attachment Option.none
              (Option.some (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isCreature)))))
        Option.none]
}
