/-- "Modular N" with its definition, a static ability and a triggered one [CR#702.43a]. -/
keyword modular (count : Amount) : Ability := {
  lexical := { lemma := "modular", parameters := [[Amount], [DashKeyword]] },
  spelling := "modular <count>",
  body :=
    [ static (entersWithCounters thisPermanent count p1p1Counter),
      when (does thisPermanent die)
        (may (putCounters (countersOn p1p1Counter thisPermanent) p1p1Counter
          (target (and [isArtifact, isCreature])))) ]
}
