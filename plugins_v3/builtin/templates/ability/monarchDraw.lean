/-- [CR#725.2] "At the beginning of the monarch's end step, that player draws a card." -/
template monarchDraw : Conferral := {
  -- TODO(spelling): At the beginning of the monarch's end step, that player draws a card.; this is
  -- a rules conferral, whose inherent or ability-free origin cannot be encoded by card-text
  -- substitution.
  body :=
    .ability (at_ (beginningOfPossessed .endStep theMonarch) (instruct (that player) (draw 1)))
}
