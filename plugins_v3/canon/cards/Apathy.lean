card := {
  name := "Apathy",
  cost := [blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    doesntUntap (enchanted creature) (controllerOf (enchanted creature)),
    at_ (beginningOfPossessed upkeep (controllerOf (enchanted creature)))
      (instruct (that player)
         (may (discard (aAtRandom (cardIn (handOf (that player))))) (untap (that creature))))
  ]
}
