/-- Authoring primitive for `StaticSpec.characteristicChange`. -/
primitive characteristicChange (subject : NounPhrase)
    (edits : List CharacteristicEdit := []) : StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.characteristicChange subject edits
}
