/-- "a card in <zone>": a card [CR#108.2] — not a token — in the zone. -/
template cardIn (zone : Location) : Predicate := {
  spelling := "card in <zone>",
  body :=
    and [isCard, in_ zone]
}
