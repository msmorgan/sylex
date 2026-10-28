card := {
  name := "Aven Shrine",
  cost := [1, white, white],
  types := [enchantment],
  text := [
    whenever (does (a isPlayer) (cast (a isSpell)))
      [
        instruct they (gainLife (letter x)),
        define x (countOf [in_ graveyard, (shares cardName share (that spell))])
      ]
  ]
}
