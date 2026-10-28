primitive actionSearch (scope : Location) (for_ : NounPhrase) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.search scope for_
}
