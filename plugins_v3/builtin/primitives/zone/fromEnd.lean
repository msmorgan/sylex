primitive fromEnd (end_ : PileEnd) (ordinal : Ordinal := nth 1) (loc : Location) : Location := {
  body := Semantics.Primitives.Location.fromEnd end_ ordinal loc
}
