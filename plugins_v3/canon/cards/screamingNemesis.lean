card := {
  name := "Screaming Nemesis",
  cost := [2, red],
  types := [creature],
  subtypes := [spirit],
  text := [
    haste,
    whenever (does any (dealDamage (to := thisCreature)))
      [
        instruct it (dealDamage thatMuch (target isAnyOtherTarget)),
        doIf (was (dealDamage anyAmount (a isPlayer)) thisWay)
          (establishThroughout (cant they (gainLife anyAmount)) restOfGame)
      ]
  ],
  power := 3,
  toughness := 3
}
