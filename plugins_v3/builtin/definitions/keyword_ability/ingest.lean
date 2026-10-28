/-- "Ingest" means "Whenever this creature deals combat damage to a player, that player exiles the
top card of their library." [CR#702.115a] -/
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
