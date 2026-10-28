/-- Malamet Battle Glyph: "Choose target creature you control and target creature you don't
control. If the creature you control entered this turn, put a +1/+1 counter on it. Then those
creatures fight each other." -/
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
