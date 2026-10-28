card := {
  name := "Oona's Blackguard",
  cost := [1, black],
  types := [creature],
  subtypes := [faerie, rogue],
  text := [
    flying,
    entersWithAdditionalCounters (each [isCreature, rogue, youControl, isOther]) 1 p1p1Counter,
    whenever
      (does (a [isCreature, youControl, hasCounter p1p1Counter])
         (dealDamage (to := a isPlayer) (kind := some combat)))
      (instruct (that player) (discard (a (cardIn hand))))
  ],
  power := 1,
  toughness := 1
}
