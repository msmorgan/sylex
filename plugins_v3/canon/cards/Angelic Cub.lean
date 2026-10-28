/-- Angelic Cub's trigger sentence: "Whenever this creature becomes the target of a spell or
ability for the first time each turn, put a +1/+1 counter on it."
Fragment: the conditional flying sentence needs preposed static-condition ordering; the
`StaticSpec.conditional` stepper currently reads the grant's "it" before its antecedent in
"As long as this creature has three or more +1/+1 counters on it". -/
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
