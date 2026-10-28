/-- Oust {W} — Sorcery. "Put target creature into its owner's library second from the top.
Its controller gains 3 life." -/
card := {
  name := "Oust",
  cost := [white],
  types := [sorcery],
  text := [spell [putInto (target isCreature) (fromTop 2 library), instruct (controllerOf it) (gainLife 3)]]
}
