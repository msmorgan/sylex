template greatestManaValue (kind : Predicate) (who : NounPhrase) : NounPhrase := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := the (and [kind, controlledBy who,
            superlative .max (.stat .manaValue) (and [kind, controlledBy who])])
}
