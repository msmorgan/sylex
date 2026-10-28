card := {
  name := "Pious Warrior",
  cost := [3, white],
  types := [creature],
  subtypes := [human, rebel, warrior],
  text := [whenever (does any (dealDamage (to := thisCreature) (kind := some combat))) (gainLife thatMuch)],
  power := 2,
  toughness := 3
}
