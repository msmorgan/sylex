/-- Naban, Dean of Iteration -/
card := {
  name := "Naban, Dean of Iteration",
  cost := [1, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    additionalTriggers
      (causes (byEvent (does (a [wizard, youControl]) enter))
         (triggers (a [abilityHead anyTriggered, abilityOf (a [isPermanent, youControl])])))
      (exactly 1)
  ],
  power := 2,
  toughness := 1
}
