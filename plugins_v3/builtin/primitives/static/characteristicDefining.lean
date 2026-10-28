primitive characteristicDefining (subject : NounPhrase)
    (edits : List CharacteristicEdit) : StaticSpec := {
  body := Semantics.Primitives.StaticSpec.characteristicDefining subject edits
}
