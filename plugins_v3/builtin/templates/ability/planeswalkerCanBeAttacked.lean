/-- [CR#306.6] "Planeswalkers can be attacked." -/
template planeswalkerCanBeAttacked : Conferral := {
  -- TODO(spelling): Planeswalkers can be attacked.; this is a rules conferral, whose inherent or
  -- ability-free origin cannot be encoded by card-text substitution.
  body :=
    .property (can any (attack this_))
}
