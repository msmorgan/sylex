card := {
  name := "Felidar Sovereign",
  cost := [4, white, white],
  types := [creature],
  subtypes := [cat, beast],
  text := [
    vigilance,
    lifelink,
    at_ (beginningOfYour upkeep) (if_ := compareAmt (lifeTotalOf you) atLeast 40) winGame
  ],
  power := 4,
  toughness := 6
}
