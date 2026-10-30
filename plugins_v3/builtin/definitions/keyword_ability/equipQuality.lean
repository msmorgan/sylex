/-- "Equip [quality] [cost]": equip whose target is a creature you control with the quality
[CR#702.6c], as "[Cost]: Attach this permanent to target [quality] creature you control. Activate
only as a sorcery." [CR#702.6a] -/
keyword equipQuality (quality : Predicate) (cost : Cost) : Ability := {
  lexical := {
    lemma := "equip",
    parameters := [[Cost], [Quality (None), Cost]],
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
  spelling := "equip <quality> <cost>",
  body :=
    [ activatedOnlyDuring cost
        (attachTo thisPermanent (target (and [quality, isCreature, youControl])))
        asSorcery ]
}
