primitive actionShiftResult (changes : List (Semantics.Delta Semantics.Amount)) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.shiftResult changes
}
