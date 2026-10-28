/-- Meletis Charlatan -/
card := {
  name := "Meletis Charlatan",
  cost := [2, blue],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    activated (compound [[2, blue], tapSymbol])
      [
        instruct (controllerOf (target [isInstantOrSorcery, isSpell])) (copyOnStack it),
        instruct (that player) (may (chooseNewTargets (that copy)))
      ]
  ],
  power := 2,
  toughness := 3
}
