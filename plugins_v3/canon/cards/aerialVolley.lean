/-- Aerial Volley {G} — Instant. "Aerial Volley deals 3 damage divided as you choose among
one, two, or three target creatures with flying." -/
card aerialVolley : Spelled :=
  { name := "Aerial Volley", cost := [pip .green], types := [.instant],
    text :=
      [ spell (dealDivided this_ 3
          (targets (oneThrough 3) (and [isCreature, hasKeyword flying]))) ] }
