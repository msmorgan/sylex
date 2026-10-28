primitive edits (edits : List CharacteristicEdit) : CopyExcept := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CopyExcept.edits edits
}
