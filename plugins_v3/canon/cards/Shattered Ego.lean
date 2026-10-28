card := {
  name := "Shattered Ego",
  cost := [blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    getsPt (enchanted creature) (down 3) (down 0),
    activated [3, blue, blue] (putInto (enchanted creature) (fromTop 3 library))
  ]
}
