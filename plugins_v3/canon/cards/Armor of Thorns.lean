/- ## Cast as though it had flash, sacrificed at cleanup

Each prints "You may cast this spell as though it had flash. If you cast it any time a sorcery
couldn't have been cast, the controller of the permanent it becomes sacrifices it at the
beginning of the next cleanup step." (`asThoughFlashSacrificedAtCleanup`). -/

card := {
  name := "Armor of Thorns",
  cost := [1, green],
  types := [enchantment],
  subtypes := [aura],
  text := [
    asThoughFlashSacrificedAtCleanup,
    enchant [isCreature, non (hasColor black)],
    getsPt (enchanted creature) (up 2) (up 2)
  ]
}
