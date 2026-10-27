/-- [CR#704.5w,310.8] "If a non-Siege battle has defense 0, it's put into its owner's
graveyard." A battle's defense on the battlefield is its defense-counter count [CR#310.4c], and
the Siege exclusion is a subtype read, not a seam. -/
template battleDefenseZero : Conferral := {

  -- TODO(spelling): If a non-Siege battle has defense 0, it's put into its owner's graveyard.; this
  -- is a rules conferral, whose inherent or ability-free origin cannot be encoded by card-text
  -- substitution.
  body :=
    Semantics.Conferral.stateBased
      (Semantics.Templates.Condition.and
        [Semantics.Templates.Condition.not
            (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.this_
              (Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.siege)),
          Semantics.Templates.Condition.compareAmt
            (Semantics.Templates.Amount.countersOn (Semantics.CounterKind.named "defenseCounter") Semantics.Templates.NounPhrase.this_)
            Semantics.Comparator.eq 0])
      (Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.this_ Semantics.Templates.Location.graveyard)
}
