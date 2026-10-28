/-- [CR#704.5g] "If a creature has toughness greater than 0, it has damage marked on it, and the
total damage marked on it is greater than or equal to its toughness, that creature has been dealt
lethal damage and is destroyed." The marked damage is `statOf markedDamage` [CR#120.6]; "is
destroyed" is the destroy action, which regeneration can replace [CR#701.19a]. -/
template lethalDamage : Conferral := {

  -- TODO(spelling): If a creature has toughness greater than 0, it has damage marked on it, and the
  -- total damage marked on it is greater than or equal to its toughness, that creature has been
  -- dealt lethal damage and is destroyed.; this is a rules conferral, whose inherent or ability-
  -- free origin cannot be encoded by card-text substitution.
  body :=
    Semantics.Conferral.stateBased
      (and
        [compareAmt (toughnessOf this_)
            Semantics.Comparator.greater 0,
          compareAmt
            (statOf Semantics.ProjAxis.markedDamage this_)
            Semantics.Comparator.greater 0,
          compareAmt
            (statOf Semantics.ProjAxis.markedDamage this_)
            Semantics.Comparator.atLeast (toughnessOf this_)])
      (destroy this_)
}
