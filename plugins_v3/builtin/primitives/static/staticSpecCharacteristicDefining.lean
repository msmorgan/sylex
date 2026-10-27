primitive staticSpecCharacteristicDefining (subject : Semantics.NounPhrase) (edits : List Semantics.CharacteristicEdit) : Semantics.StaticSpec := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.StaticSpec.characteristicDefining subject edits
}
