/-- Aerial Volley {G} — Instant. "Aerial Volley deals 3 damage divided as you choose among
one, two, or three target creatures with flying." -/
card := {
  name := "Aerial Volley",
  cost := [green],
  types := [instant],
  text := [spell (dealDivided this_ 3 (targets (oneThrough 3) [isCreature, hasKeyword flying]))]
}
