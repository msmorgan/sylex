/-- [CR#120.3h] "Damage dealt to a battle causes that many defense counters to be removed from
that battle." -/
template battleDefenseDamage : Conferral := {

  -- TODO(spelling): Damage dealt to a battle causes that many defense counters to be removed from
  -- that battle.; this is a rules conferral, whose inherent or ability-free origin cannot be
  -- encoded by card-text substitution.
  body :=
    Semantics.Templates.Conferral.damageResult
      (Semantics.Templates.Action.loseCounters
        (Option.some (Semantics.Templates.CounterKindSource.printedKind (Semantics.CounterKind.named "defenseCounter")))
        (Option.some Semantics.Templates.Amount.thatMuch) Semantics.Templates.NounPhrase.this_)
}
