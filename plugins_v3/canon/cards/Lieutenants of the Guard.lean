card := {
  name := "Lieutenants of the Guard",
  cost := [4, white],
  types := [creature],
  subtypes := [human, soldier],
  text := [
    councilsDilemma
      (when (does thisCreature enter)
         [
           instruct eachPlayer (voteStartingWith you openly (byLabel ["strength", "numbers"])),
           putCounters (votesFor "strength") p1p1Counter thisCreature,
           create (votesFor "numbers") (creatureToken 1 1 [white] [soldier])
         ])
  ],
  power := 2,
  toughness := 2
}
