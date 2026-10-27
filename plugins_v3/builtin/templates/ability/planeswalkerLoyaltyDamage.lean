/-- [CR#120.3c] "Damage dealt to a planeswalker causes that many loyalty counters to be removed
from that planeswalker." -/
template planeswalkerLoyaltyDamage : Conferral := {

  -- TODO(spelling): Damage dealt to a planeswalker causes that many loyalty counters to be removed
  -- from that planeswalker.; this is a rules conferral, whose inherent or ability-free origin
  -- cannot be encoded by card-text substitution.
  body :=
    Semantics.Templates.Conferral.damageResult
      (Semantics.Templates.Action.loseCounters
        (Option.some (Semantics.Templates.CounterKindSource.printedKind (Semantics.CounterKind.named "loyaltyCounter")))
        (Option.some Semantics.Templates.Amount.thatMuch) Semantics.Templates.NounPhrase.this_)
}
