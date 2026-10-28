card := {
  name := "Consul's Lieutenant",
  cost := [white, white],
  types := [creature],
  subtypes := [human, soldier],
  text := [
    firstStrike,
    renown 1,
    whenever (does thisCreature attack) (if_ := is it (hasDesignation renowned))
      (get (allOf [isCreature, isAttacking, youControl, isOther]) (up 1) (up 1) untilEndOfTurn)
  ],
  power := 2,
  toughness := 1
}
