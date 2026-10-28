/-- Katara, the Fearless -/
card := {
  name := "Katara, the Fearless",
  cost := [green, white, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, warrior, ally],
  text := [
    additionalTriggers (triggers (a [abilityHead anyTriggered, abilityOf (a [ally, youControl])]))
      (exactly 1)
  ],
  power := 3,
  toughness := 3
}
