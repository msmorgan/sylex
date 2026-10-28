/-- Reconfigure [CR#702.151a]: attaching is the `attachTo` Definition; unattaching moves the
Equipment away and is no attaching [CR#701.3d], so it is the bare attachment change and names no
deed. -/
keyword reconfigure (cost : Cost) : Ability := {
  lexical := { lemma := "reconfigure", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): reconfigure [mana cost] / reconfigure—[verbal cost]; separator and terminal
  -- period depend on the Cost structure.
  body :=
    [activatedOnlyDuring cost
        (attachTo thisPermanent
          (target
            (and
              [isCreature, isOther, youControl])))
        asSorcery,
      abilityActivated cost (unattach thisPermanent)
        (some asSorcery) none
        (some
          (is thisPermanent
            (predicateAttachment Semantics.AttachmentSide.attachment none
              (some (a isCreature)))))
        none]
}
