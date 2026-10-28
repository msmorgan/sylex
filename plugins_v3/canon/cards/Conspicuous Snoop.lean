/-- Conspicuous Snoop -/
card := {
  name := "Conspicuous Snoop",
  cost := [red, red],
  types := [creature],
  subtypes := [goblin, rogue],
  text := [
    visibility reveal you topOfLibrary,
    can you (play (allOf [isSpell, goblin]) (from_ := onTop)),
    onlyWhile (abilityGrantFrom thisCreature (topSlice 1) [anyActivated]) (is (topSlice 1) goblin)
  ],
  power := 2,
  toughness := 2
}
