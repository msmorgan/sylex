/-- Chromatic Armor -/
card := {
  name := "Chromatic Armor",
  cost := [1, white, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    entersChoosing thisAura color,
    entersWithCounters thisAura 1 sleightCounter,
    replace
      (would (by_ (allOf [isSource, ofTheLastChosen color]))
         (dealDamage anyAmount (enchanted creature)))
      (instead (preventDamage allDamage)),
    activated (mana [«variable»])
      [
        putCounters 1 sleightCounter thisAura,
        choose (a (quality color)),
        define x (countersOn sleightCounter thisAura)
      ]
  ]
}
