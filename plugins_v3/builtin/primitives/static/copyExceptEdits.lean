primitive copyExceptEdits (edits : List Semantics.CharacteristicEdit) : Semantics.CopyExcept := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CopyExcept.edits edits
}
