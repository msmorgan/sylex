primitive repetitionFixed (times : Semantics.Amount) (body : Semantics.Action) : Semantics.Repetition := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Repetition.fixed times body
}
