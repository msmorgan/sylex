card := {
  name := "Kaervek's Purge",
  cost := [«variable», black, red],
  types := [sorcery],
  text := [
    spell
      [
        destroy (target [isCreature, hasManaValue eq (letter x)]),
        doIf (did (that creature) die thisWay)
          (instruct this_ (dealDamage (powerOf (that creature)) (controllerOf (that creature))))
      ]
  ]
}
