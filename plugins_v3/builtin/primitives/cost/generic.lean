primitive generic (change : Delta Amount) (floor : Option Amount) : CostShift := {
  body := Semantics.Primitives.CostShift.generic change floor
}
