card := {
  name := "Oracle of Mul Daya",
  cost := [3, green],
  types := [creature],
  subtypes := [elf, shaman],
  text := [
    can you (play (counted (additional (exactly 1)) isLand)) (timing := onEachOfYourTurns),
    visibility reveal you topOfLibrary,
    can you (play (allOf isLand) (from_ := onTop))
  ],
  power := 2,
  toughness := 2
}
