/-- Atris, Oracle of Half-Truths -/
card := {
  name := "Atris, Oracle of Half-Truths",
  cost := [2, blue, black],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, advisor],
  text := [
    menace,
    when (does thisCreature enter)
      [
        instruct (target isOpponent) (lookAt (topSlice 3)),
        instruct they (separateIntoPiles them 2 [faceDown, faceUp]),
        putInto onePile hand,
        putInto theOther graveyard
      ]
  ],
  power := 3,
  toughness := 2
}
