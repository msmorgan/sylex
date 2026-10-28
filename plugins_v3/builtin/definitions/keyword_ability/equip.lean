/-- "Equip [cost]" means "[Cost]: Attach this permanent to target creature you control. Activate
only as a sorcery." [CR#702.6a] -/
keyword equip (cost : Cost) : Ability := {
  lexical := {
    lemma := "equip",
    parameters := [[Cost], [Quality (None), Cost], [DashNonManaCost]],
    determinatives := [{ number := Singular, countability := Count, lemma := "equipped" }],
    verbs := [
               {
                 lemma := "equip",
                 past := "equipped",
                 gerund := "equipping",
                 frames := [Monotransitive]
               }
             ]
  },
  -- TODO(spelling): equip [mana cost] / equip—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [ activatedOnlyDuring cost
        (attachTo thisPermanent (target (and [isCreature, youControl])))
        asSorcery ]
}
