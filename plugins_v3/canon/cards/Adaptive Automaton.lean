/-- Adaptive Automaton. "This creature is the chosen type in addition to its other types" is a
Characteristic-Defining Ability: it depends on the choice made as it enters, as Shapeshifter's
power does, and depending on a choice is no condition [CR#604.3a]. Its pump of the other creatures of the chosen type stays a
separate ability, affecting other objects [CR#604.3a]. -/
card := {
  name := "Adaptive Automaton",
  cost := [3],
  types := [artifact, creature],
  subtypes := [construct],
  text := [
    entersChoosing thisCreature (subtype creature),
    qualityDefinition thisCreature adds (chosenQuality (ofChosen (subtype creature))),
    getsPt (allOf [isCreature, youControl, isOther, ofChosen (subtype creature)]) (up 1) (up 1)
  ],
  power := 2,
  toughness := 2
}
