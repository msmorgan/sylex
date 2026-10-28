card := {
  name := "Labyrinth of Skophos",
  types := [land],
  text := [
    activated tapSymbol (addMana 1 (manaOf [[colorless]])),
    activated (compound [[4], tapSymbol])
      (removeFromCombat (target [isCreature, or [isAttacking, isBlocking]]))
  ]
}
