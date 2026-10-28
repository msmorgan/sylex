card := {
  name := "Inferno Elemental",
  cost := [4, red, red],
  types := [creature],
  subtypes := [elemental],
  text := [
    whenever (does thisCreature (block (a isCreature)))
      (or_ := [does (by_ (a isCreature)) (block thisCreature)])
      (instruct thisCreature (dealDamage 3 (that creature)))
  ],
  power := 4,
  toughness := 4
}
