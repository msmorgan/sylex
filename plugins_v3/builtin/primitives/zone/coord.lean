/-- Full-arity binding for `Location.coord`. -/
primitive coord (op : Coordinator) (locs : List Location) : Location := {
  body := Semantics.Primitives.Location.coord op locs
}
