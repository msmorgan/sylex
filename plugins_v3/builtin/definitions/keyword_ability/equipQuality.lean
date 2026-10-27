/-- "Equip [quality] [cost]": equip whose target is a creature you control with the quality
[CR#702.6c], as "[Cost]: Attach this permanent to target [quality] creature you control. Activate
only as a sorcery." [CR#702.6a] -/
keyword equipQuality (quality : Predicate) (cost : Cost) : Ability := {
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
  label := "equip",
  -- TODO(spelling): equip [quality] [mana cost] / equip [quality]—[verbal cost]. The separator
  -- and terminal period depend on the Cost structure.
  body :=
    [ activatedOnlyDuring cost
        (attachTo thisPermanent (target (and [quality, isCreature, youControl])))
        asSorcery ]
}
