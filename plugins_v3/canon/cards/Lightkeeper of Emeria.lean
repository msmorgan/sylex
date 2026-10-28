card := {
  name := "Lightkeeper of Emeria",
  cost := [3, white],
  types := [creature],
  subtypes := [angel],
  text := [
    multikicker [white],
    flying,
    when (does thisCreature enter) (gainLife (times 2 (timesPaid (byKeyword kicker) thisCreature)))
  ],
  power := 2,
  toughness := 4
}
