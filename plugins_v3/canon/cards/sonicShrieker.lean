card := {
  name := "Sonic Shrieker",
  cost := [2, red, white, black],
  types := [creature],
  subtypes := [dragon],
  text := [
    flying,
    when (does thisCreature enter)
      [
        instruct it (dealDamage 2 (target isAnyTarget)),
        gainLife 2,
        doIf (was (dealDamage anyAmount (a isPlayer)) thisWay)
          (instruct they (discard (a (cardIn hand))))
      ]
  ],
  power := 4,
  toughness := 4
}
