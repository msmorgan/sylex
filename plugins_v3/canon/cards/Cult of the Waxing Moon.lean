card := {
  name := "Cult of the Waxing Moon",
  cost := [4, green],
  types := [creature],
  subtypes := [human, shaman],
  text := [
    whenever
      (does any
         (transform (a [isPermanent, youControl])
            (into := some [isCreature, not (hasSubtype human)])))
      (create 1 (creatureToken 2 2 [green] [wolf]))
  ],
  power := 5,
  toughness := 4
}
