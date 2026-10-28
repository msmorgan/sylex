card := {
  name := "Undercover Butler",
  cost := [2, hybridPip blue black],
  types := [creature],
  subtypes := [human, rogue],
  text := [
    whenever
      (does thisCreature (attack (a [isPlayer, superlative max (playerStat lifeTotal) isPlayer])))
      (establishThroughout (cant any (block it)) thisTurn)
  ],
  power := 2,
  toughness := 3
}
