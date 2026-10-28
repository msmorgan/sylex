card := {
  name := "Tower Winder",
  cost := [1, green],
  types := [creature],
  subtypes := [snake],
  text := [
    reach,
    deathtouch,
    when (does thisCreature enter)
      [
        searchLibraryOrGraveyard (a (named "Command Tower")),
        reveal it,
        putInto it hand,
        doIf (did you (search yourLibrary) thisWay) shuffle
      ]
  ],
  power := 1,
  toughness := 1
}
