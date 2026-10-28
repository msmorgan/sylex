/-- Delivery Moogle -/
card := {
  name := "Delivery Moogle",
  cost := [3, white],
  types := [creature],
  subtypes := [moogle],
  text := [
    flying,
    when (does thisCreature enter)
      [
        searchLibraryOrGraveyard (a [isArtifact, hasManaValue atMost 2]),
        reveal it,
        putInto it hand,
        doIf (did you (search yourLibrary) thisWay) shuffle
      ]
  ],
  power := 3,
  toughness := 2
}
