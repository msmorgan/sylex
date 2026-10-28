card := {
  name := "Éomer of the Riddermark",
  cost := [4, red],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, knight],
  text := [
    haste,
    whenever (does thisCreature attack)
      (if_ := exists_
         [isCreature, youControl, superlative max (stat power) [isCreature, isPermanent]])
      (create 1 (creatureToken 1 1 [white] [human, soldier]))
  ],
  power := 5,
  toughness := 4
}
