/-- "A permanent with indestructible can't be destroyed." [CR#702.12b] -/
keyword indestructible : Ability := {
  lexical := { lemma := "indestructible" },
  spelling := "indestructible",
  body :=
    [static
        (cant any (destroy thisPermanent))]
}
