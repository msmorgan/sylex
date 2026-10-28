/-- Soul Ransom -/
card := {
  name := "Soul Ransom",
  cost := [2, blue, black],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    controlGrant you (enchanted creature),
    activatedBy (repeatTimes 2 [choose (a (cardIn hand)), discard (that card)])
      [instruct (controllerOf thisAura) (sacrifice it), instruct they (draw 2)] yourOpponents
  ]
}
