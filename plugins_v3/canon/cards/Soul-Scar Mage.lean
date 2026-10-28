card := {
  name := "Soul-Scar Mage",
  cost := [red],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    prowess,
    replace
      (would (a [isSource, youControl])
         (dealDamage anyAmount (a [isCreature, anOpponentControls]) (some noncombat)))
      (instead (putCounters thatMuch m1m1Counter (that creature)))
  ],
  power := 1,
  toughness := 2
}
