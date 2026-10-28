primitive characteristicDefining (subject : NounPhrase)
    (edits : List CharacteristicEdit) : StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.characteristicDefining subject edits
}
