/-- [CR#704.5i] "If a planeswalker has loyalty 0, it's put into its owner's graveyard."
A planeswalker's loyalty on the battlefield is its loyalty-counter count [CR#306.5c,122.1e],
so the condition reads the counters and not the printed characteristic. -/
template loyaltyZero : Conferral := {

  -- TODO(spelling): If a planeswalker has loyalty 0, it's put into its owner's graveyard.; this is
  -- a rules conferral, whose inherent or ability-free origin cannot be encoded by card-text
  -- substitution.
  body :=
    Semantics.Conferral.stateBased
      (compareAmt
        (countersOn (Semantics.CounterKind.named "loyaltyCounter") this_)
        Semantics.Comparator.eq 0)
      (putInto this_ graveyard)
}
