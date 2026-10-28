card := {
  name := "Harsh Sustenance",
  cost := [1, white, black],
  types := [instant],
  text := [
    spell
      [
        instruct this_ (dealDamage (letter x) (target isAnyTarget)),
        gainLife (letter x),
        define x (countOf [isCreature, youControl])
      ]
  ]
}
