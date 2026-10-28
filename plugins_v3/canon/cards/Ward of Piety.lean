card := {
  name := "Ward of Piety",
  cost := [1, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    activated [1, white]
      (establishThroughout
         (replace (would any (dealDamage anyAmount (enchanted creature)))
            (instead (redirectDamage (theNext 1) (target isAnyTarget))))
         thisTurn)
  ]
}
