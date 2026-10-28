card := {
  name := "Aggravated Assault",
  cost := [2, red],
  types := [enchantment],
  text := [
    activatedOnlyDuring [3, red, red]
      [
        untap (allOf [isCreature, youControl]),
        additionalPhase combat (some mainPhase) 1 (some mainPhase)
      ]
      asSorcery
  ]
}
