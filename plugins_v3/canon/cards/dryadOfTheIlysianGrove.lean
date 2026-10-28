card := {
  name := "Dryad of the Ilysian Grove",
  cost := [2, green],
  types := [enchantment, creature],
  subtypes := [nymph, dryad],
  text := [
    can you (play (counted (additional (exactly 1)) isLand)) (timing := onEachOfYourTurns),
    qualityChange (allOf [isLand, youControl]) adds (everyTypeOf basicLand)
  ],
  power := 2,
  toughness := 4
}
