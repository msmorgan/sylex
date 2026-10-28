/-- [CR#310.4b] "A battle has the intrinsic ability 'This permanent enters with a number of
defense counters on it equal to its printed defense number.'" The battle row of the same shape
[CR#614.1c], counting the battle's defense, which off the battlefield is its printed number
[CR#310.4a]. -/
template battleDefense : Conferral := {

  -- TODO(spelling): A battle has the intrinsic ability 'This permanent enters with a number of
  -- defense counters on it equal to its printed defense number.'; this is a rules conferral, whose
  -- inherent or ability-free origin cannot be encoded by card-text substitution.
  body :=
    Semantics.Conferral.ability
      (static
        (entersWithCounters this_
          (statOf (Semantics.ProjAxis.stat Semantics.Stat.defense) this_)
          (Semantics.CounterKind.named "defenseCounter")))
}
