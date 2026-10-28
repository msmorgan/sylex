/-- Menacing Ogre -/
card := {
  name := "Menacing Ogre",
  cost := [3, red, red],
  types := [creature],
  subtypes := [ogre],
  text := [
    trample,
    haste,
    when (does thisCreature enter)
      [
        instruct eachPlayer (choose (disclosure := secretly) (a (quality number))),
        revealChoices numbers,
        instruct (each [isPlayer, choseExtreme max]) (loseLife thatMuch),
        doIf (is you (choseExtreme max)) (putCounters 2 p1p1Counter thisCreature)
      ]
  ],
  power := 3,
  toughness := 3
}
