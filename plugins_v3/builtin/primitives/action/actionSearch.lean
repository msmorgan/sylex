/-- Search a zone for a card matching the description [CR#701.23a]. The actor performs the search. -/
primitive actionSearch (scope : Location) (for_ : NounPhrase) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.search scope for_
}
