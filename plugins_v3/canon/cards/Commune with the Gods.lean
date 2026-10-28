card := {
  name := "Commune with the Gods",
  cost := [1, green],
  types := [sorcery],
  text := [
    spell
      [
        reveal (topSlice 5),
        may (putInto (fromAmong (exactly 1) (or [isCreature, isEnchantment]) them) hand),
        putInto theRest graveyard
      ]
  ]
}
