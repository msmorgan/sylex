/-- Reconfigure [CR#702.151a]: attaching is the `attachTo` Definition; unattaching moves the
Equipment away and is no attaching [CR#701.3d], so it is the bare attachment change and names no
deed. -/
keyword reconfigure (cost : Cost) : Ability := {
  lexical := { lemma := "reconfigure", parameters := [[Cost]] },
  spelling := "reconfigure <cost>",
  body :=
    [activatedOnlyDuring cost
        (attachTo thisPermanent
          (target
            (and
              [isCreature, isOther, youControl])))
        asSorcery,
      activated cost (unattach thisPermanent) (some asSorcery) none (some
          (is thisPermanent
            (attachment .attachment (some (a isCreature)) none))) none]
}
