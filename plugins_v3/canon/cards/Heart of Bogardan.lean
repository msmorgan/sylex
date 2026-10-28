/-- Heart of Bogardan -/
card := {
  name := "Heart of Bogardan",
  cost := [2, red, red],
  types := [enchantment],
  text := [
    cumulativeUpkeep [2],
    when
      (does (a isPlayer) (pay anyCost once unpaid (some thisEnchantment) (some cumulativeUpkeep)))
      [
        simultaneously
          [
            instruct this_ (dealDamage (letter x) (target (or [isPlaneswalker, isPlayer]))),
            instruct this_
              (dealDamage (letter x) (each [isCreature, controlledBy splitOverPlaneswalker]))
          ],
        define x (minus (times 2 (countersOn ageCounter thisEnchantment)) 2)
      ]
  ]
}
