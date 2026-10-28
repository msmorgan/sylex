card := {
  name := "Bramblewood Paragon",
  cost := [1, green],
  types := [creature],
  subtypes := [elf, warrior],
  text := [
    entersWithAdditionalCounters (each [isCreature, warrior, youControl, isOther]) 1 p1p1Counter,
    abilityGrant (each [isCreature, youControl, hasCounter p1p1Counter]) trample
  ],
  power := 2,
  toughness := 2
}
