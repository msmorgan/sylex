/-- Bioplasm -/
card := {
  name := "Bioplasm",
  cost := [3, green, green],
  types := [creature],
  subtypes := [ooze],
  text := [
    whenever (does thisCreature attack)
      [
        exile (topSlice 1),
        doIf (is it [isCard, isCreature])
          (get thisCreature (up (powerOf (the [was (exile which), isCreature, isCard])))
             (up (toughnessOf it)) thisTurn)
      ]
  ],
  power := 4,
  toughness := 4
}
