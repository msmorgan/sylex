/-- Burn, Burn, Tree and Fern -/
card := {
  name := "Burn, Burn, Tree and Fern",
  cost := [3, red],
  types := [enchantment],
  subtypes := [saga],
  text := [
    when (chapterMark [1]) (instruct this_ (dealDamage 6 (target [isCreature, anOpponentControls]))),
    when (chapterMark [2]) (destroy (target [isArtifact, anOpponentControls])),
    when (chapterMark [3, 4]) (addMana 1 (manaOf [[of red]]))
  ]
}
