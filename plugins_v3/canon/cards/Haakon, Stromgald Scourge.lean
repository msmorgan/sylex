card := {
  name := "Haakon, Stromgald Scourge",
  cost := [1, black, black],
  supertypes := [legendary],
  types := [creature],
  subtypes := [zombie, knight],
  text := [
    can you (cast this_ (from_ := yourGraveyard)),
    cant you (cast this_ (from_ := some (not yourGraveyard))),
    onlyWhile (can you (cast (bare [isSpell, knight]) (from_ := yourGraveyard)))
      (is this_ (in_ battlefield)),
    when (does thisCreature die) (loseLife 2)
  ],
  power := 3,
  toughness := 3
}
