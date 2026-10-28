/-- Fall of the Titans -/
card := {
  name := "Fall of the Titans",
  cost := [«variable», «variable», red],
  types := [instant],
  text := [
    surge [«variable», red],
    spell (instruct this_ (dealDamage (letter x) (eachOf (targets (upTo 2) isAnyTarget))))
  ]
}
