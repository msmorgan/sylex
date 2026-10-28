/-- Wormfang Manta {5}{U}{U} — Creature — Nightmare Fish Beast 6/1. "Flying. When Wormfang
Manta enters, you skip your next turn. When Wormfang Manta leaves the battlefield, you take an
extra turn after this one." -/
card := {
  name := "Wormfang Manta",
  cost := [5, blue, blue],
  types := [creature],
  subtypes := [nightmare, fish, beast],
  text := [
    flying,
    when (does thisCreature enter) (skipPart turn 1),
    when (does thisCreature leave) (extraTurn 1)
  ],
  power := 6,
  toughness := 1
}
