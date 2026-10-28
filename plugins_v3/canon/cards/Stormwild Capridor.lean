card := {
  name := "Stormwild Capridor",
  cost := [2, white],
  types := [creature],
  subtypes := [bird, goat],
  text := [
    flying,
    replace (would any (dealDamage anyAmount thisCreature (some noncombat)))
      (instead
         (preventDamage allDamage (also := putCounters preventedThisWay p1p1Counter thisCreature)))
  ],
  power := 1,
  toughness := 3
}
