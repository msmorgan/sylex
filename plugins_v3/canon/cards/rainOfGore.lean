/-- Rain of Gore -/
card rainOfGore : Spelled :=
  { name := "Rain of Gore", cost := [pip .black, pip .red], types := [.enchantment],
    text :=
      [ static (replace
    (causedBy
      (bySource (a (or [isSpell,
        abilityHead .anyOnStack])))
      (would
        (controllerOf it)
        (gainLife anyAmount)))
    (instead (instruct (that player) (loseLife thatMuch)))) ] }
