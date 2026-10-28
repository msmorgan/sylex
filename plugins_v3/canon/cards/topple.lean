/-- Topple {2}{W} — Sorcery. "Exile target creature with the greatest power among creatures
on the battlefield." -/
card := {
  name := "Topple",
  cost := [2, white],
  types := [sorcery],
  text := [spell (exile (target [isCreature, superlative max (stat power) [isCreature, isPermanent]]))]
}
