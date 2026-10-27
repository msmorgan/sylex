/-- [CR#302.5] "Creatures can attack and block." -/
template creatureCanBlock : Conferral := {
  -- TODO(spelling): Creatures can attack and block.; this is a rules conferral, whose inherent or
  -- ability-free origin cannot be encoded by card-text substitution.
  body :=
    .property (can this_ block)
}
