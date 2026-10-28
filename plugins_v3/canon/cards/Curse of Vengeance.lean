card := {
  name := "Curse of Vengeance",
  cost := [black],
  types := [enchantment],
  subtypes := [aura, curse],
  text := [
    enchant isPlayer,
    whenever (does (enchanted player) (cast (a isSpell))) (putCounters 1 spiteCounter thisAura),
    when (does (enchanted player) loseGame)
      [gainLife (letter x), draw (letter x), define x (countersOn spiteCounter thisAura)]
  ]
}
