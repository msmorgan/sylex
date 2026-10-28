keyword ingest : Ability := {
  lexical := { lemma := "ingest" },
  spelling := "ingest",
  body :=
    [whenever
        (does (actor thisCreature)
          (dealDamage anyAmount (a isPlayer)
            (some Semantics.DamageKind.combat)))
        (instruct they
          (exile
            (slice Semantics.PileEnd.top 1 (libraryOf they))))]
}
