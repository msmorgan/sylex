primitive nounPhraseOneEachOf (roles : List Semantics.Predicate) (pool : Semantics.NounPhrase) : Semantics.NounPhrase := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.NounPhrase.oneEachOf roles pool
}
