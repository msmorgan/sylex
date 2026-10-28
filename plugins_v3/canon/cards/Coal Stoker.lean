card := {
  name := "Coal Stoker",
  cost := [3, red],
  types := [creature],
  subtypes := [elemental],
  text := [
    when (does thisCreature enter)
      (if_ := is it (did you (cast which (from_ := yourHand)) thisObject))
      (addMana 1 (manaOf [[of red, of red, of red]]))
  ],
  power := 3,
  toughness := 3
}
