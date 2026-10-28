template cardIn (zone : Location) : Predicate := {
  spelling := "card in <zone>",
  body :=
    and [isCard, in_ zone]
}
