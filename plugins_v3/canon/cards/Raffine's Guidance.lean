card := {
  name := "Raffine's Guidance",
  cost := [white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    getsPt (enchanted creature) (up 1) (up 1),
    can you (cast this_ (from_ := yourGraveyard) (payment := payingInstead [2, white]))
  ]
}
