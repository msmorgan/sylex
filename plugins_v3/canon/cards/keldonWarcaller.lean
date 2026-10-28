card := {
  name := "Keldon Warcaller",
  cost := [1, red],
  types := [creature],
  subtypes := [human, warrior],
  text := [whenever (does thisCreature attack) (putCounters 1 loreCounter (target [saga, youControl]))],
  power := 2,
  toughness := 2
}
