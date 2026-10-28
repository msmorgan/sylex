/-- Danitha, New Benalia's Light -/
card := {
  name := "Danitha, New Benalia's Light",
  cost := [1, green, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, knight],
  text := [
    vigilance,
    trample,
    lifelink,
    can you (cast (counted (exactly 1) [isSpell, or [aura, equipment]]) (from_ := yourGraveyard))
      (timing := onEachOfYourTurns)
  ],
  power := 2,
  toughness := 2
}
