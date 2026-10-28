/-- Orchard Elemental -/
card := {
  name := "Orchard Elemental",
  cost := [5, green],
  types := [creature],
  subtypes := [elemental],
  text := [
    councilsDilemma
      (when (does thisCreature enter)
         [
           instruct eachPlayer (voteStartingWith you openly (byLabel ["sprout", "harvest"])),
           putCounters (times 2 (votesFor "sprout")) p1p1Counter thisCreature,
           gainLife (times 3 (votesFor "harvest"))
         ])
  ],
  power := 2,
  toughness := 2
}
