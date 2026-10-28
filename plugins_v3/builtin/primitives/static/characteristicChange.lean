/-- Authoring primitive for `StaticSpec.characteristicChange`. -/
primitive characteristicChange (subject : NounPhrase)
    (edits : List CharacteristicEdit := []) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.characteristicChange subject edits
}
