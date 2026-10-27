/-- [CR#310.5] "Battles can be attacked." -/
template battleCanBeAttacked : Conferral := {
  -- TODO(spelling): Battles can be attacked.; this is a rules conferral, whose inherent or ability-
  -- free origin cannot be encoded by card-text substitution.
  body :=
    .property (can any (attack this_))
}
