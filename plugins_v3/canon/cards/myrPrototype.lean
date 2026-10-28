/-- Myr Prototype -/
card := {
  name := "Myr Prototype",
  cost := [5],
  types := [artifact, creature],
  subtypes := [myr],
  text := [
    at_ (beginningOfYour upkeep) (putCounters 1 p1p1Counter thisCreature),
    static
      [
        gatedBy (scaledGenericMana (times 1 (countersOn p1p1Counter it))) thisCreature attack,
        gatedBy (scaledGenericMana (times 1 (countersOn p1p1Counter it))) thisCreature block
      ]
  ],
  power := 2,
  toughness := 2
}
