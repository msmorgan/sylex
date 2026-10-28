/-- [CR#306.5b] "A planeswalker has the intrinsic ability 'This permanent enters with a number
of loyalty counters on it equal to its printed loyalty number.'" A replacement effect
[CR#614.1c] whose count is the planeswalker's loyalty, `statOf (.stat .loyalty) .this`: off the
battlefield a planeswalker card's loyalty is its printed number [CR#306.5a], so the printed
number needs no projection of its own. When the entering object is read is the engine's
concern, not the term's (ADR §0.1 items 4 and 5; owner ruling 2026-10-22). -/
template planeswalkerLoyalty : Conferral := {

  -- TODO(spelling): A planeswalker has the intrinsic ability 'This permanent enters with a number
  -- of loyalty counters on it equal to its printed loyalty number.'; this is a rules conferral,
  -- whose inherent or ability-free origin cannot be encoded by card-text substitution.
  body :=
    Semantics.Conferral.ability
      (static
        (entersWithCounters this_
          (statOf (Semantics.ProjAxis.stat Semantics.Stat.loyalty) this_)
          (Semantics.CounterKind.named "loyaltyCounter")))
}
