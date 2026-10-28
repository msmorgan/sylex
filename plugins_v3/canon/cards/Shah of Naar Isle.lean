card := {
  name := "Shah of Naar Isle",
  cost := [3, red],
  types := [creature],
  subtypes := [efreet],
  text := [
    trample,
    echo [0],
    when (does any (pay anyCost once paid (some thisCreature) (some echo)))
      (instruct eachOpponent (may (drawCount (upToOf 3))))
  ],
  power := 6,
  toughness := 6
}
