/-- [CR#302.6] "A creature's activated ability with the tap symbol or the untap symbol in its
activation cost can't be activated unless the creature has been under its controller's control
continuously since their most recent turn began." Haste is the exception [CR#702.10c]. -/
template summoningSicknessActivation : Conferral := {
  -- TODO(spelling): A creature's activated ability with the tap symbol or the untap symbol in its
  -- activation cost can't be activated unless the creature has been under its controller's control
  -- continuously since their most recent turn began.; this is a rules conferral, whose inherent or
  -- ability-free origin cannot be encoded by card-text substitution.
  body :=
    .property
      (onlyUnless
        (cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf this_,
          tapOrUntapInCost]))))
        (controlledSinceTheirTurnBegan this_))
}
