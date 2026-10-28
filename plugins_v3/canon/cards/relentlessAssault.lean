card := {
  name := "Relentless Assault",
  cost := [2, red, red],
  types := [sorcery],
  text := [
    spell
      [
        untap (allOf [isCreature, did which attack (within soFarThisTurn)]),
        additionalPhase combat (some mainPhase) 1 (some mainPhase)
      ]
  ]
}
