/-- [CR#120.3c] "Damage dealt to a planeswalker causes that many loyalty counters to be removed
from that planeswalker." -/
template planeswalkerLoyaltyDamage : Conferral := {
  -- TODO(spelling): Damage dealt to a planeswalker causes that many loyalty counters to be removed
  -- from that planeswalker.; this is a rules conferral, whose inherent or ability-free origin
  -- cannot be encoded by card-text substitution.
  body :=
    damageResult (loseCounters (some (.printed (.named "loyaltyCounter"))) (some .thatMuch) .this)
}
