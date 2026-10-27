/-- "Level up [cost]" with its definition, an activated ability: "[Cost]: Put a level counter on
this permanent. Activate only as a sorcery." [CR#702.87a] -/
keyword levelUp (cost : Cost) : Ability := {
  lexical := { lemma := "level up", parameters := [[Cost]] },
  spelling := "level up <cost>",
  body :=
    [ activatedOnlyDuring cost
        (putCounters 1 levelCounter thisPermanent) asSorcery ]
}
