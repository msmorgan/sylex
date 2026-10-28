card := {
  name := "Colossal Grave-Reaver",
  cost := [6, black, green],
  types := [creature],
  subtypes := [dragon],
  text := [
    flying,
    whenever (does thisCreature enter) (or_ := [does thisCreature attack]) (mill 3),
    whenever
      (does any
         (putInto (counted (orMore 1) [isCard, isCreature]) yourGraveyard (from_ := yourLibrary)))
      (putOntoBattlefield (someOf (exactly 1) them))
  ],
  power := 7,
  toughness := 6
}
