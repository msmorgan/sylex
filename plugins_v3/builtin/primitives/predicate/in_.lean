/-- "in <loc>", as "in your graveyard" or "exiled with this artifact". -/
primitive in_ (loc : Location) : Predicate := {
  spelling := "in <loc>",
  body :=
    Semantics.Primitives.Predicate.in_ loc
}
