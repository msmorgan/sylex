/-- Blood Frenzy -/
card := {
  name := "Blood Frenzy",
  cost := [1, red],
  types := [instant],
  text := [
    spell
      [
        get (target [isCreature, or [isAttacking, isBlocking]]) (up 4) (up 0) untilEndOfTurn,
        atNext endStep (destroy (that creature))
      ]
      (timing := beforePart combatDamage none)
  ]
}
