/-- Thallid -/
card := {
  name := "Thallid",
  cost := [green],
  types := [creature],
  subtypes := [fungus],
  text := [
    at_ (beginningOfYour upkeep) (putCounters 1 sporeCounter thisCreature),
    activated (removeCounters (exactly 3) sporeCounter thisCreature)
      (create 1 (creatureToken 1 1 [green] [saproling]))
  ],
  power := 1,
  toughness := 1
}
