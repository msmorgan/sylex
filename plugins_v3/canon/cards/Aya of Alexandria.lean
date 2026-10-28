card := {
  name := "Aya of Alexandria",
  cost := [2, red, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, assassin],
  text := [
    menace,
    lifelink,
    whenever
      (does (a [isHistoric, isCreature, youControl])
         (dealDamage (to := a isPlayer) (kind := some combat)))
      (create 1
         (written
            {
              colors := [black],
              types := [creature],
              subtypes := [assassin],
              text := [menace],
              power := 1,
              toughness := 1
            }))
  ],
  power := 4,
  toughness := 3
}
