/-- "a creature type other than <subtype>" -/
primitive typeOtherThan (subtype : Subtype) : ChoiceDomain := {
  spelling := "a creature type other than <subtype>",
  body :=
    Semantics.Primitives.ChoiceDomain.typeOtherThan subtype
}
