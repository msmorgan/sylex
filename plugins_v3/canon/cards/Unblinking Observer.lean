card := {
  name := "Unblinking Observer",
  cost := [1, blue],
  types := [creature],
  subtypes := [homunculus],
  text := [
    activated tapSymbol
      (addMana 1 (manaOf [[of blue]])
         [spendOnly [toPay (ofKeyword disturb), toCast isInstantOrSorcery]])
  ],
  power := 2,
  toughness := 1
}
