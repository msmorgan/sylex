/-- Captain Marvel, Apex Avenger -/
card := {
  name := "Captain Marvel, Apex Avenger",
  cost := [5, red, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, kree, hero],
  text := [
    flying,
    doubleStrike,
    indestructible,
    whenever (does (by_ you) (putCounters anyAmount anyKind (a [isCreature, isOther])))
      (if_ := is it (not kree)) (may (putCounters thatMuch thoseCounters this_))
  ],
  power := 4,
  toughness := 4
}
