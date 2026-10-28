/-- Syr Konrad, the Grim -/
card := {
  name := "Syr Konrad, the Grim",
  cost := [3, black, black],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, knight],
  text := [
    whenever (does (a [isCreature, isOther]) die)
      (or_ :=
         [
           does any (putInto (a [isCard, isCreature]) graveyard (from_ := some (not battlefield))),
           does (a [isCreature, in_ yourGraveyard]) (leave (from_ := yourGraveyard))
         ])
      (instruct this_ (dealDamage 1 eachOpponent)),
    activated [1, black] (instruct eachPlayer (mill 1))
  ],
  power := 5,
  toughness := 4
}
