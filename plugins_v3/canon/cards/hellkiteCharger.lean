card := {
  name := "Hellkite Charger",
  cost := [4, red, red],
  types := [creature],
  subtypes := [dragon],
  text := [
    flying,
    haste,
    whenever (does thisCreature attack)
      (may (pay [5, red, red] once)
         [untap (allOf [isCreature, isAttacking]), additionalPhase combat none 1])
  ],
  power := 5,
  toughness := 5
}
