/-- "Except it has <values>", the types `typesAdded` "in addition to its other types". -/
primitive characteristics (values : Characteristics) (typesAdded : Bool) : CopyExcept := {
  body := Semantics.Primitives.CopyExcept.characteristics values typesAdded
}
