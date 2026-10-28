/-- Shimmering Glasskite -/
card := {
  name := "Shimmering Glasskite",
  cost := [3, blue],
  types := [creature],
  subtypes := [spirit],
  text := [
    flying,
    whenever
      (nthOccurrence (nth 1) (does (a (or [isSpell, abilityHead anyOnStack])) (target thisCreature))
         (some turn))
      (counter (that stack))
  ],
  power := 2,
  toughness := 3
}
