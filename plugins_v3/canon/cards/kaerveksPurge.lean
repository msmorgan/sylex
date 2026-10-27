/-- Kaervek's Purge: "Destroy target creature with mana value X. If that creature dies this way,
Kaervek's Purge deals damage equal to the creature's power to the creature's controller." -/
card kaerveksPurge : Spelled :=
  { name := "Kaervek's Purge", cost := [.variable, pip .black, pip .red], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ destroy (target (and [isCreature, hasManaValue .eq (letter .x)])),
            doIf (did (that creature) die thisWay)
              (instruct this_
                (dealDamage (powerOf (that creature)) (controllerOf (that creature)))) ]) ] }
