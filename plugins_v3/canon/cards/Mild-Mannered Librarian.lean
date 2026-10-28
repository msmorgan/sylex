card := {
  name := "Mild-Mannered Librarian",
  cost := [green],
  types := [creature],
  subtypes := [human],
  text := [
    activatedOnlyOnce [3, green]
      [
        establish (qualityChange thisCreature sets (bundle { subtypes := [werewolf] })),
        putCounters 2 p1p1Counter it,
        draw 1
      ]
      oncePerGame
  ],
  power := 1,
  toughness := 1
}
