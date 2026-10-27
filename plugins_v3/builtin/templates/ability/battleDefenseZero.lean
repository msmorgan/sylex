/-- [CR#704.5w,310.8] "If a non-Siege battle has defense 0, it's put into its owner's
graveyard." A battle's defense on the battlefield is its defense-counter count [CR#310.4c], and
the Siege exclusion is a subtype read, not a seam. -/
template battleDefenseZero : Conferral := {
  -- TODO(spelling): If a non-Siege battle has defense 0, it's put into its owner's graveyard.; this
  -- is a rules conferral, whose inherent or ability-free origin cannot be encoded by card-text
  -- substitution.
  body :=
    .stateBased
      (.and [ .not (.matches .this (.hasSubtype siege)),
              .compareAmt (countersOn (.named "defenseCounter") .this) .eq 0 ])
      (putInto .this graveyard)
}
