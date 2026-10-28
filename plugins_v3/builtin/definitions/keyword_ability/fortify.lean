/-- "Fortify [cost]" means "[Cost]: Attach this Fortification to target land you control. Activate
only as a sorcery." [CR#702.67a] -/
keyword fortify (cost : Cost) : Ability := {
  lexical := {
    lemma := "fortify",
    determinatives := [{ number := Singular, countability := Count, lemma := "fortified" }],
    parameters := [[Cost]],
    verbs := [{ lemma := "fortify", frames := [Monotransitive] }]
  },
  spelling := "fortify <cost>",
  body :=
    [ activatedOnlyDuring cost
        (attachTo thisPermanent (target (and [isLand, youControl])))
        asSorcery ]
}
