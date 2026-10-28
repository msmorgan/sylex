card := {
  name := "Fry",
  cost := [1, red],
  types := [instant],
  text := [
    cant any (counter thisSpell),
    spell
      (instruct this_
         (dealDamage 5
            (target [or [isCreature, isPlaneswalker], or [hasColor white, hasColor blue]])))
  ]
}
