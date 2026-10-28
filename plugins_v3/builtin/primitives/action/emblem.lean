primitive emblem (name : Option String) (abilities : List Ability) : CreationSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CreationSpec.emblem name abilities
}
