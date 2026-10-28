primitive actionShiftResult (changes : List (Delta Amount)) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.shiftResult changes
}
