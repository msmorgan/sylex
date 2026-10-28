/-- Vexing Shusher -/
card := {
  name := "Vexing Shusher",
  cost := [hybridPip red green, hybridPip red green],
  types := [creature],
  subtypes := [goblin, shaman],
  text := [
    cant any (counter thisSpell),
    activated [hybridPip red green] (establish (cant any (counter (target isSpell))))
  ],
  power := 2,
  toughness := 2
}
