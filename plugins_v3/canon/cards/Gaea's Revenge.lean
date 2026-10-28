card := {
  name := "Gaea's Revenge",
  cost := [5, green, green],
  types := [creature],
  subtypes := [elemental],
  text := [
    cant any (counter thisSpell),
    haste,
    cant
      (by_
         (allOf
            (or
               [[isSpell, non (hasColor green)], abilityOf (allOf [isSource, non (hasColor green)])])))
      (target thisCreature)
  ],
  power := 8,
  toughness := 5
}
