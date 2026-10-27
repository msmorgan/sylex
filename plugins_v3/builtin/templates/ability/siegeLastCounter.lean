/-- [CR#310.12b] "Sieges have the intrinsic ability 'When the last defense counter is removed
from this permanent, exile it, then you may cast it transformed without paying its mana cost.'"
Cast transformed, the resulting spell has its back face up [CR#712.8c]. -/
template siegeLastCounter : Conferral := {
  -- TODO(spelling): Sieges have the intrinsic ability 'When the last defense counter is removed
  -- from this permanent, exile it, then you may cast it transformed without paying its mana cost.';
  -- this is a rules conferral, whose inherent or ability-free origin cannot be encoded by card-text
  -- substitution.
  body :=
    .ability
      (when (does any (removeCounters theLast defenseCounter thisPermanent))
        (sequentially
          [ exile it,
            may (cast it (payment := withoutPaying) (resulting := some isTransformed)) ]))
}
