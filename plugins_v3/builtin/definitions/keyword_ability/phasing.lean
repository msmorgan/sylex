/-- Phasing modifies the untap step: before untapping, phased-in permanents with phasing controlled
by the active player phase out, while permanents that phased out under that player’s control
phase in. [CR#702.26a] -/
keyword phasing : Ability := {
  lexical := { lemma := "phasing" },
  spelling := "phasing",
  body :=
    []
}
