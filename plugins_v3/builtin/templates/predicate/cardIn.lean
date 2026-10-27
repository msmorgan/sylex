template cardIn (zone : Location) : Predicate := {
  spelling := "card in <zone>",
  body :=
    Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCard, Semantics.Templates.Predicate.in_ zone]
}
