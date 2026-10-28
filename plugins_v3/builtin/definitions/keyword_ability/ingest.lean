keyword ingest : Ability := {
  lexical := { lemma := "ingest" },
  spelling := "ingest",
  body :=
    [whenever
        (does (actor thisCreature)
          (dealDamage anyAmount (a isPlayer)
            (some .combat)))
        (instruct they
          (exile
            (slice .top 1 (libraryOf they))))]
}
