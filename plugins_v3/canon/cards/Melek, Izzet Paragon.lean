card := {
  name := "Melek, Izzet Paragon",
  cost := [4, blue, red],
  supertypes := [legendary],
  types := [creature],
  subtypes := [weird, wizard],
  text := [
    visibility reveal you topOfLibrary,
    can you (cast (bare [isSpell, isInstantOrSorcery]) (from_ := onTop)),
    whenever (does you (cast (a [isInstantOrSorcery, isSpell]) (from_ := yourLibrary)))
      [copyOnStack it, may (chooseNewTargets (that copy))]
  ],
  power := 2,
  toughness := 4
}
