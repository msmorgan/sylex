card := {
  name := "Wilt-Leaf Liege",
  cost := [1, hybridPip green white, hybridPip green white, hybridPip green white],
  types := [creature],
  subtypes := [elf, knight],
  text := [
    getsPt (allOf [isCreature, hasColor green, youControl, isOther]) (up 1) (up 1),
    getsPt (allOf [isCreature, hasColor white, youControl, isOther]) (up 1) (up 1),
    replace
      (causedBy (bySource (a [or [isSpell, abilityHead anyOnStack], anOpponentControls]))
         (would you (discard thisCard)))
      (instead (putOntoBattlefield it))
  ],
  power := 4,
  toughness := 4
}
