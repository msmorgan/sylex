/-- A token described by an explicit characteristic bundle. -/
primitive written (characteristics : CharacteristicBundle) : TokenSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.TokenSpec.written characteristics
}
