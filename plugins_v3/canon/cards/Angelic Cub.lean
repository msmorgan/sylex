/-- "Whenever this creature becomes the target of a spell or ability for the first time each turn,
put a +1/+1 counter on it": "for the first time each turn" watches only the first matching trigger
event each turn [CR#603.2], and "it" in the effect resolves to `thisCreature` rather than the
targeting spell or ability. -/
card := {
  name := "Angelic Cub",
  cost := [1, white],
  types := [creature],
  subtypes := [cat, angel],
  text := [
    whenever
      (nthOccurrence (nth 1) (does (a (or [isSpell, abilityHead anyOnStack])) (target thisCreature))
         (some turn))
      (putCounters 1 p1p1Counter it)
  ],
  power := 1,
  toughness := 1
}
