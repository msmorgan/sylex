primitive fixed (times : Amount) (body : Action) : Repetition := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Repetition.fixed times body
}
