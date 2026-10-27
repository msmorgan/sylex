primitive creationSpecEmblem (name : Option String) (abilities : List Semantics.Ability) : Semantics.CreationSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CreationSpec.emblem name abilities
}
