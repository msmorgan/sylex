/-- [CR#704.5f] "If a creature has toughness 0 or less, it's put into its owner's graveyard." -/
template toughnessZero : Conferral := {

  -- TODO(spelling): If a creature has toughness 0 or less, it's put into its owner's graveyard.;
  -- this is a rules conferral, whose inherent or ability-free origin cannot be encoded by card-text
  -- substitution.
  body :=
    Semantics.Conferral.stateBased
      (Semantics.Templates.Condition.compareAmt (Semantics.Templates.Amount.toughnessOf Semantics.Templates.NounPhrase.this_)
        Semantics.Comparator.atMost 0)
      (Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.this_ Semantics.Templates.Location.graveyard)
}
