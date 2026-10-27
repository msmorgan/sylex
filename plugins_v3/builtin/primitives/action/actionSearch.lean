primitive actionSearch (scope : Semantics.Location) (for_ : Semantics.NounPhrase) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.search scope for_
}
