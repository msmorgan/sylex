primitive nounPhraseSomeOf (count : Semantics.SliceCount) (description : Option Semantics.Predicate) (group : Semantics.NounPhrase) : Semantics.NounPhrase := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.NounPhrase.someOf count description group
}
