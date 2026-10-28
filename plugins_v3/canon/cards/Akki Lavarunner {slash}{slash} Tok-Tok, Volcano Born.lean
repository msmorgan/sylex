card := flip
  {
    normal := {
      name := "Akki Lavarunner",
      cost := [3, red],
      types := [creature],
      subtypes := [goblin, warrior],
      text := [haste, whenever (does thisCreature (dealDamage (to := anOpponent))) (flip thisCreature)],
      power := 1,
      toughness := 1
    },
    alternative := {
      name := "Tok-Tok, Volcano Born",
      supertypes := [legendary],
      types := [creature],
      subtypes := [goblin, shaman],
      text := [
        protection (hasColor red),
        replace (would (a [isSource, hasColor red]) (dealDamage anyAmount (a isPlayer)))
          (instead (instruct it (dealDamage (plus thatMuch 1) (that player))))
      ],
      power := 2,
      toughness := 2
    }
  }
