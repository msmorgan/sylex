/-- Kaervek's Purge: "Destroy target creature with mana value X. If that creature dies this way,
Kaervek's Purge deals damage equal to the creature's power to the creature's controller." -/
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
