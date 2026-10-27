primitive predicateChosen (ref : Semantics.ChoiceRef) : Semantics.Predicate := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Predicate.chosen ref
}
