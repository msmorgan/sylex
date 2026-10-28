primitive nounPhraseSomeOf (count : SliceCount) (description : Option Predicate) (group : NounPhrase) : NounPhrase := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.NounPhrase.someOf count description group
}
