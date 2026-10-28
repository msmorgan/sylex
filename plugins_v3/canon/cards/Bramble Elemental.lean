card := {
  name := "Bramble Elemental",
  cost := [3, green, green],
  types := [creature],
  subtypes := [elemental],
  text := [
    whenever (does any (attachment attached (a aura) thisCreature))
      (create 2 (creatureToken 1 1 [green] [saproling]))
  ],
  power := 4,
  toughness := 4
}
