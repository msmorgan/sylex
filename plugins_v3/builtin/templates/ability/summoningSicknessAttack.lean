/-- [CR#302.6] "A creature can't attack unless it has been under its controller's control
continuously since their most recent turn began." Haste is the exception [CR#702.10b]. -/
template summoningSicknessAttack : Conferral := {
  -- TODO(spelling): A creature can't attack unless it has been under its controller's control
  -- continuously since their most recent turn began.; this is a rules conferral, whose inherent or
  -- ability-free origin cannot be encoded by card-text substitution.
  body :=
    .property (onlyUnless (cant this_ attack) (controlledSinceTheirTurnBegan it))
}
