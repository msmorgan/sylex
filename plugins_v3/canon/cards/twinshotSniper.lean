/-- Twinshot Sniper -/
card := {
  name := "Twinshot Sniper",
  cost := [3, red],
  types := [artifact, creature],
  subtypes := [goblin, archer],
  text := [
    reach,
    when (does thisCreature enter) (instruct it (dealDamage 2 (target isAnyTarget))),
    channel
      (activated (compound [[1, red], discard this_])
         (instruct it (dealDamage 2 (target isAnyTarget))))
  ],
  power := 2,
  toughness := 3
}
