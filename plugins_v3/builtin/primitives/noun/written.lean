/-- A token described by an explicit characteristic bundle. -/
primitive written (characteristics : CharacteristicBundle) : TokenSpec := {
  body :=
    Semantics.Primitives.TokenSpec.written characteristics
}
