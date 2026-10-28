card := {
  name := "Aisling Leprechaun",
  cost := [green],
  types := [creature],
  subtypes := [faerie],
  text := [
    whenever (does thisCreature (block (a isCreature)))
      (or_ := [does (by_ (a isCreature)) (block thisCreature)])
      (becomeColor (that creature) (.some [green]))
  ],
  power := 1,
  toughness := 1
}
