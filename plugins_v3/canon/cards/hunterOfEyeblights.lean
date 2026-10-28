card := {
  name := "Hunter of Eyeblights",
  cost := [3, black, black],
  types := [creature],
  subtypes := [elf, assassin],
  text := [
    when (does thisCreature enter) (putCounters 1 p1p1Counter (target [isCreature, youDontControl])),
    activated (compound [[2, black], tapSymbol]) (destroy (target [isCreature, hasCounter]))
  ],
  power := 3,
  toughness := 3
}
