card := {
  name := "Gaddock Teeg",
  cost := [green, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [kithkin, advisor],
  text := [
    cant any (cast (bare [isSpell, non isCreature, hasManaValue atLeast 4])),
    cant any (cast (bare [isSpell, non isCreature, manaCostHas «variable»]))
  ],
  power := 2,
  toughness := 2
}
