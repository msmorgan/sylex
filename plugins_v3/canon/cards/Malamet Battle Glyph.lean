card := {
  name := "Malamet Battle Glyph",
  cost := [green],
  types := [sorcery],
  text := [
    spell
      [
        (choose (both (target [isCreature, youControl]) (target [isCreature, youDontControl]))),
        doIf (did (the [isCreature, youControl]) enter soFarThisTurn) (putCounters 1 p1p1Counter it),
        fightEachOther (those creature)
      ]
  ]
}
