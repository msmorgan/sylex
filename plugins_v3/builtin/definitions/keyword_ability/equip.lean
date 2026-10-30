/-- "Equip [cost]" means "[Cost]: Attach this permanent to target creature you control. Activate
only as a sorcery." [CR#702.6a] -/
keyword equip (cost : Cost) : Ability := {
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
  spelling := "equip <cost>",
  body :=
    [ activatedOnlyDuring cost
        (attachTo thisPermanent (target (and [isCreature, youControl])))
        asSorcery ]
}
