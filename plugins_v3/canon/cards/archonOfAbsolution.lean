/-- Archon of Absolution -/
card := {
  name := "Archon of Absolution",
  cost := [3, white],
  types := [creature],
  subtypes := [archon],
  text := [
    flying,
    protection (hasColor white),
    gatedBy (scaledGenericMana (times 1 groupSize)) (allOf isCreature)
      (attack (youOr (allOf [isPlaneswalker, youControl])))
  ],
  power := 3,
  toughness := 2
}
