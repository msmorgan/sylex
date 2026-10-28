/-- [CR#120.3h] "Damage dealt to a battle causes that many defense counters to be removed from
that battle." -/
template battleDefenseDamage : Conferral := {

  -- TODO(spelling): Damage dealt to a battle causes that many defense counters to be removed from
  -- that battle.; this is a rules conferral, whose inherent or ability-free origin cannot be
  -- encoded by card-text substitution.
  body :=
    damageResult
      (loseCounters
        (some (printedKind (Semantics.CounterKind.named "defenseCounter")))
        (some thatMuch) this_)
}
