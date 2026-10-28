card := {
  name := "Autarch Mammoth",
  cost := [4, green, green],
  types := [creature],
  subtypes := [elephant, mount],
  text := [
    (when (does thisCreature enter)
       (joins :=
          [
            joinedHeadWhile (does thisCreature attack)
              (whileTrue (is thisCreature (hasDesignation saddled)))
          ])
       (create 1 (creatureToken 3 3 [green] [elephant]))),
    saddle 5
  ],
  power := 5,
  toughness := 5
}
