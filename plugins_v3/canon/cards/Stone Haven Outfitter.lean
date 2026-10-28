card := {
  name := "Stone Haven Outfitter",
  cost := [1, white],
  types := [creature],
  subtypes := [kor, artificer, ally],
  text := [
    getsPt (allOf [[isCreature, youControl], isAttached (some equipped)]) (up 1) (up 1),
    whenever (does (a [[isCreature, youControl], isAttached (some equipped)]) die) (draw 1)
  ],
  power := 2,
  toughness := 2
}
